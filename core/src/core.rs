use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;

use crate::api::{Request, Response};
use crate::graph::Graph;
use crate::io::{Input, Output};
use crate::node::{Node, Position};
use crate::package::{Engine, Package};
use crate::ExecuteFn;
use macrograph_package_api::engine::{EngineContext, Event};
use macrograph_package_api::schema::NodeSchemaType;
use macrograph_package_api::ExecuteContext;
use macrograph_package_api::package::Package as ApiPackage;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tokio::sync::oneshot;

pub struct Core {
    pub graphs: HashMap<i32, Graph>,
    pub packages: Vec<Package>,
    request_channel: (
        UnboundedSender<WrappedRequest>,
        UnboundedReceiver<WrappedRequest>,
    ),
    event_channel: (UnboundedSender<Event>, UnboundedReceiver<Event>),
    graph_id_counter: i32,
}

struct WrappedRequest {
    inner: Request,
    sender: oneshot::Sender<Response>,
}

impl WrappedRequest {
    fn new(request: Request, sender: oneshot::Sender<Response>) -> Self {
        Self {
            inner: request,
            sender,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ConnectIOError {
    InvalidNodes { input: bool, output: bool },
    InvalidIO { input: bool, output: bool },
}

pub struct CoreController {
    request_sender: UnboundedSender<WrappedRequest>,
}

impl CoreController {
    pub async fn send(&self, request: Request) -> Response {
        let (sender, recv) = oneshot::channel();
        let wrapped = WrappedRequest::new(request, sender);

        self.request_sender.send(wrapped);

        let resp = recv.await;

        resp.unwrap()
    }
}

impl Core {
    pub fn new() -> Self {
        let mut ret = Self {
            graphs: HashMap::new(),
            packages: vec![],
            request_channel: unbounded_channel(),
            event_channel: unbounded_channel(),
            graph_id_counter: 0,
        };

        ret.create_graph("Graph 0".into());

        ret
    }

    pub fn create_graph(&mut self, name: String) -> i32 {
        let id = self.graph_id_counter;
        self.graph_id_counter += 1;
        self.graphs.insert(id, Graph::new(id, name));
        id
    }

    pub fn graph(&self, id: i32) -> Option<&Graph> {
        self.graphs.get(&id)
    }

    pub fn graph_mut(&mut self, id: i32) -> Option<&mut Graph> {
        self.graphs.get_mut(&id)
    }

    pub fn load_library(&mut self, path: &str) {
        unsafe {
            let lib = libloading::Library::new(path).unwrap();
            let create_package: libloading::Symbol<fn() -> ApiPackage> =
                lib.get(b"create_package").unwrap();
            let p = create_package();
            self.packages.push(p.into());
        }
    }

    pub fn package(&self, name: &str) -> Option<&Package> {
        self.packages.iter().find(|p| p.name == name)
    }

    pub fn package_mut(&mut self, name: &str) -> Option<&mut Package> {
        self.packages.iter_mut().find(|p| p.name == name)
    }

    pub fn setup(&mut self) {
        for package in &mut self.packages {
            package.engine = if let Some(engine) = package.engine.take() {
                match engine {
                    Engine::Created { run, state } => {
                        let (request_sender, request_receiver) = unbounded_channel();
                        let handle = package.runtime.handle();
                        let handle = handle.clone();
                        let package = package.name.clone();
                        let event_sender = self.event_channel.0.clone();

                        std::thread::spawn(move || {
                            run(EngineContext {
                                initial_state: state,
                                request_receiver,
                                handle: handle.clone(),
                                package,
                                event_sender,
                            });
                        });

                        Some(Engine::Running { request_sender })
                    }
                    _ => {
                        println!("Attempted to setup an engine that has already been created");
                        None
                    }
                }
            } else {
                None
            }
        }
    }

    async fn process_request(&mut self, request: WrappedRequest) {
        use Request::*;

        let res = match request.inner {
            CreateNode {
                graph,
                package,
                schema,
                position,
            } => {
                let node = self
                    .create_node(graph, &package, &schema, position)
                    .await
                    .unwrap();
                let inputs = node.inputs.lock().unwrap();
                let outputs = node.outputs.lock().unwrap();

                Response::CreateNode {
                    id: node.id,
                    inputs: inputs.iter().map(|i| i.into()).collect(),
                    outputs: outputs.iter().map(|o| o.into()).collect(),
                }
            }
            DeleteNode { graph, node } => {
                self.delete_node(graph, node);
                Response::DeleteNode
            }
            ConnectIO {
                graph,
                output_node,
                output,
                input_node,
                input,
            } => {
                self.connect_io(graph, output_node, &output, input_node, &input)
                    .unwrap();

                Response::ConnectIO
            }
            DisconnectIO {
                graph,
                node,
                io,
                is_input,
            } => {
                self.disconnect_io(graph, node, &io, is_input).unwrap();

                Response::DisconnectIO
            }
            SetDefaultValue {
                graph,
                node,
                input,
                value,
            } => {
                let input = self
                    .graph(graph)
                    .unwrap()
                    .node(node)
                    .unwrap()
                    .find_data_input(&input)
                    .unwrap();

                input.set_default_value(value);

                Response::SetDefaultValue
            }
            GetPackages => Response::GetPackages {
                packages: self.packages.iter().map(|p| p.into()).collect(),
            },
            GetProject => Response::GetProject {
                graphs: self.graphs.values().map(|g| g.into()).collect(),
            },
            Reset { graph } => {
                self.graph_mut(graph).unwrap().reset();
                Response::Reset
            }
        };

        request.sender.send(res);
    }

    pub(crate) async fn create_node(
        &mut self,
        graph: i32,
        package: &str,
        schema: &str,
        position: Position,
    ) -> Option<Arc<Node>> {
        let schema = self.package(&package).unwrap().schema(&schema); //and_then(|p| p.schema(&schema));

        if let Some(schema) = schema {
            let schema = schema.clone();
            let node = self
                .graph_mut(graph)
                .unwrap()
                .create_node(&schema, position);
            let mut instances = schema.instances.lock().await;

            instances.insert(node.clone());

            return Some(node);
        }

        None
    }

    fn delete_node(&mut self, graph: i32, node: i32) {
        self.graph_mut(graph).unwrap().delete_node(node)
    }

    pub fn connect_io(
        &mut self,
        graph: i32,
        output_node: i32,
        output: &str,
        input_node: i32,
        input: &str,
    ) -> Result<(), ConnectIOError> {
        let output_node = self.graph(graph).unwrap().node(output_node);
        let input_node = self.graph(graph).unwrap().node(input_node);

        let (output_node, input_node) = match (output_node, input_node) {
            (Some(output_node), Some(input_node)) => (output_node, input_node),
            (input, output) => {
                return Err(ConnectIOError::InvalidNodes {
                    input: input.is_none(),
                    output: output.is_none(),
                });
            }
        };

        let (output, input) = match (
            output_node.find_output(output),
            input_node.find_input(input),
        ) {
            (Some(output), Some(input)) => (output, input),
            (output, input) => {
                return Err(ConnectIOError::InvalidIO {
                    input: input.is_none(),
                    output: output.is_none(),
                });
            }
        };

        match (output, input) {
            (Output::Data(output), Input::Data(input)) => {
                output.connect_input(&input);
                input.connect_output(&output);
            }
            (Output::Exec(output), Input::Exec(input)) => {
                output.connect_input(&input);
                input.connect_output(&output);
            }
            _ => {}
        };

        Ok(())
    }

    pub fn disconnect_io(
        &mut self,
        graph: i32,
        node: i32,
        io: &str,
        is_input: bool,
    ) -> Result<(), ()> {
        let node = self.graph(graph).unwrap().node(node);

        let node = match node {
            Some(node) => node,
            None => return Err(()),
        };

        match is_input {
            true => {
                let input = node.find_input(io);

                match input {
                    Some(input) => {
                        input.disconnect();
                    }
                    None => return Err(()),
                }
            }
            false => {
                let output = node.find_output(io);

                match output {
                    Some(output) => {
                        output.disconnect();
                    }
                    None => return Err(()),
                }
            }
        };

        Ok(())
    }

    pub async fn start(&mut self) {
        loop {
            tokio::select! {
                Some(request) = self.request_channel.1.recv() => {
                    self.process_request(request).await;
                }
                Some(event) = self.event_channel.1.recv() => {
                    self.handle_event(event).await;
                }
            }
        }
    }

    pub async fn handle_event(&self, event: Event) {
        let schema = self.package(&event.package).unwrap().schema(&event.event);

        if let Some(schema) = schema {
            let nodes = schema.instances.lock().await;

            let futures: Vec<_> = nodes
                .iter()
                .map(|node| self.fire_node(node.graph_id, node.id, &event.data))
                .collect();

            futures::future::join_all(futures).await;
        }
    }

    pub(crate) async fn execute_node(&self, node: &Arc<Node>) -> Option<&'static str> {
        for input in node.inputs.lock().unwrap().iter() {
            if let Input::Data(input) = input {
                if let Some(output) = input.connected_output.lock().unwrap().upgrade() {
                    match ***output.node.upgrade().unwrap().schema {
                        NodeSchemaType::Exec { .. } | NodeSchemaType::Event { .. } => {
                            input.set_value(output.value.load_full())
                        }
                        _ => {}
                    };
                } else {
                    input.reset_value();
                }
            }
        }

        let package = self.package(&node.schema.package).unwrap();
        let request_send_channel = match &package.engine {
            Some(Engine::Running { request_sender }) => request_sender.clone(),
            _ => {
                println!("Tried to execute node before engine ran");
                return None;
            }
        };

        let context = ExecuteContext::new(request_send_channel, package.runtime.handle().clone());

        let mut io_data = node.get_io_data();

        let res = match &***node.schema {
            NodeSchemaType::Base { execute } => match execute {
                ExecuteFn::Sync(execute) => execute(&mut io_data, context),
                ExecuteFn::Async(execute) => execute(&mut io_data, context).await,
            },
            NodeSchemaType::Exec { execute } => {
                match execute {
                    ExecuteFn::Sync(execute) => execute(&mut io_data, context),
                    ExecuteFn::Async(execute) => execute(&mut io_data, context).await,
                };
                Some("")
            }
            _ => None,
        };

        node.parse_io_data(io_data);

        res
    }

    pub(crate) async fn fire_node(
        &self,
        graph: i32,
        node_id: i32,
        data: &Box<dyn Any + Send + Sync>,
    ) {
        let node = self.graph(graph).unwrap().nodes.get(&node_id).unwrap();

        let fire = match ***node.schema {
            NodeSchemaType::Event { fire } => fire,
            _ => return,
        };

        let mut io_data = node.get_io_data();

        let mut target_output_mut =
            (fire)(&mut io_data, data.as_ref()).and_then(|id| node.find_exec_output(id));

        node.parse_io_data(io_data);

        while let Some(target_output) = target_output_mut.as_ref() {
            let connected_input = {
                let o = target_output.clone();
                let i = &*o.connected_input.lock().unwrap();
                i.clone()
            };

            target_output_mut = if let Some(connected_input) = connected_input.upgrade() {
                let node = connected_input.node.upgrade().unwrap();

                self.execute_node(&node)
                    .await
                    .and_then(|id| node.find_exec_output(id))
            } else {
                None
            }
        }
    }

    pub fn get_controller(&self) -> CoreController {
        CoreController {
            request_sender: self.request_channel.0.clone(),
        }
    }
}
