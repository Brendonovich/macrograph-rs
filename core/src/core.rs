use crate::api::{Request, Response};
use crate::node::Position;
use crate::schema::NodeSchemaType;
use crate::{
    EngineContext, Event, ExecuteContext, ExecuteOutput, Graph, Input, NodeRef, Output, Package,
};
use serde_json::Value;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tokio::sync::oneshot;

pub struct Core {
    pub graph: Graph,
    pub packages: Vec<Package>,
    request_channel: (
        UnboundedSender<WrappedRequest>,
        UnboundedReceiver<WrappedRequest>,
    ),
    event_channel: (UnboundedSender<Event>, UnboundedReceiver<Event>),
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
        Self {
            graph: Graph::new(),
            packages: vec![],
            request_channel: unbounded_channel(),
            event_channel: unbounded_channel(),
        }
    }

    pub fn register_package(&mut self, package: Package) {
        self.packages.push(package);
    }

    pub fn package(&self, name: &str) -> Option<&Package> {
        self.packages.iter().find(|p| p.name == name)
    }

    pub fn package_mut(&mut self, name: &str) -> Option<&mut Package> {
        self.packages.iter_mut().find(|p| p.name == name)
    }

    pub async fn start_engines(&self) {
        for package in &self.packages {
            if let Some(engine_arc) = &package.engine {
                let engine = engine_arc.lock().await;
                let run = engine.run;

                tokio::spawn(run(
                    engine_arc.clone(),
                    EngineContext::new(&package.name, &self.event_channel.0),
                ));
            }
        }
    }

    async fn process_request(&mut self, request: WrappedRequest) {
        use Request::*;

        let res = match request.inner {
            CreateNode {
                package,
                schema,
                position,
            } => {
                let node = self.create_node(&package, &schema, position).await.unwrap();
                let inputs = node.inputs.lock().unwrap();
                let outputs = node.outputs.lock().unwrap();

                Response::CreateNode {
                    id: node.id,
                    name: node.name.clone(),
                    inputs: inputs.iter().map(|i| i.into()).collect(),
                    outputs: outputs.iter().map(|o| o.into()).collect(),
                }
            }
            ConnectIO {
                output_node,
                output,
                input_node,
                input,
            } => {
                self.connect_io(output_node, &output, input_node, &input)
                    .unwrap();

                Response::ConnectIO
            }
            DisconnectIO { node, id, is_input } => {
                self.disconnect_io(node, &id, is_input).unwrap();

                Response::DisconnectIO
            }
            SetDefaultValue { node, input, value } => {
                let input = self
                    .graph
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
            Reset => {
                self.graph.reset();
                Response::Reset
            }
        };

        request.sender.send(res);
    }

    pub async fn create_node(
        &mut self,
        package: &str,
        schema: &str,
        position: Position,
    ) -> Option<NodeRef> {
        let schema = self.package(&package).unwrap().schema(&schema); //and_then(|p| p.schema(&schema));

        if let Some(schema) = schema {
            let schema = schema.clone();
            let node = self.graph.create_node(&schema, position);
            let mut instances = schema.instances.lock().await;

            instances.insert(node.clone());

            return Some(node);
        }

        None
    }

    pub fn connect_io(
        &mut self,
        output_node: i32,
        output: &str,
        input_node: i32,
        input: &str,
    ) -> Result<(), ConnectIOError> {
        let output_node = self.graph.node(output_node);
        let input_node = self.graph.node(input_node);

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

    pub fn disconnect_io(&mut self, node: i32, io: &str, is_input: bool) -> Result<(), ()> {
        let node = self.graph.node(node);

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
                .map(|node| self.fire_node(node.id, &event.data))
                .collect();

            futures::future::join_all(futures).await;
        }
    }

    pub(crate) async fn execute_node(&self, node: NodeRef) -> ExecuteOutput {
        for input in node.inputs.lock().unwrap().iter() {
            if let Input::Data(input) = input {
                if let Some(output) = &*input.connected_output.lock().unwrap() {
                    match **output.node.schema {
                        NodeSchemaType::Exec { .. } | NodeSchemaType::Event { .. } => {
                            input.set_value(output.get_value())
                        }
                        _ => {}
                    };
                } else {
                    input.reset_value();
                }
            }
        }

        let execute = match &**node.schema {
            NodeSchemaType::Exec { execute } => execute,
            NodeSchemaType::Base { execute } => execute,
            _ => return None,
        };

        let engine = self.package(&node.schema.package).unwrap().engine.clone();
        let context = ExecuteContext { engine };

        (execute)(node.clone(), context).await
    }

    pub(crate) async fn fire_node(&self, node_id: i32, data: &Value) {
        let node = self.graph.nodes.get(&node_id).unwrap();

        let fire = match &**node.schema {
            NodeSchemaType::Event { fire } => fire,
            _ => return,
        };

        let res = (fire)(node.clone(), data.clone());

        let mut next_node_mut = res
            .and_then(|o| o.connected_input.lock().unwrap().clone())
            .map(|i| i.node.clone());

        while let Some(next_node) = next_node_mut.clone() {
            let next_output = self.execute_node(next_node).await;

            next_node_mut = match next_output {
                Some(next_output) => next_output
                    .connected_input
                    .lock()
                    .unwrap()
                    .as_ref()
                    .map(|i| i.node.clone()),
                None => None,
            };
        }
    }

    pub fn get_controller(&self) -> CoreController {
        CoreController {
            request_sender: self.request_channel.0.clone(),
        }
    }
}
