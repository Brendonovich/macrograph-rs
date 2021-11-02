#![feature(const_generics)]

use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

use self::{
    engine::{EngineContext, Event},
    graph::Graph,
    package::Package,
    types::EngineRef,
};
use crate::core::types::{NodeRef, ExecuteOutput};
use serde_json::Value;
use crate::core::schema::{NodeSchema, ExecuteFn, ExecuteContext};
use crate::core::io::{Input, Output};
use std::collections::HashMap;
use std::sync::Mutex;

pub mod engine;
pub mod graph;
pub mod io;
pub mod node;
pub mod package;
pub mod schema;
pub mod types;
pub mod value;

pub struct Core {
    pub graph: Graph,
    pub packages: Vec<Package>,
    pub request_channel: (UnboundedSender<Request>, UnboundedReceiver<Request>),
    pub event_map: HashMap<(String, String), Vec<NodeRef>>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub enum RequestData {
    Event(Event),
    CreateNode {
        package: String,
        schema: String,
    },
    ConnectIO {
        input_node: i32,
        input: String,
        output_node: i32,
        output: String,
    },
    Stop,
}

#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub enum ResponseData {
    NodeCreated,
}

pub struct Request {
    pub id: i32,
    pub data: RequestData,
}

pub struct Response {
    pub id: i32,
    pub data: ResponseData,
}

#[derive(Debug, PartialEq)]
pub enum ConnectIOError {
    InvalidNodes {
        input: bool,
        output: bool,
    },
    InvalidIO {
        input: bool,
        output: bool,
    },
}

impl Core {
    pub fn new() -> Self {
        Self {
            graph: Graph::new(),
            packages: vec![],
            request_channel: unbounded_channel(),
            event_map: HashMap::new(),
        }
    }

    pub fn register_package(&mut self, package: Package) {
        self.packages.push(package);
    }

    pub fn package(&self, name: &str) -> Option<&Package> {
        self.packages.iter().find(|p| p.name == name)
    }

    pub async fn start_engines(&self) {
        for package in &self.packages {
            if let Some(engine) = &package.engine {
                let start = engine.clone().lock().unwrap().start;

                start(
                    engine.clone(),
                    EngineContext::new(
                        &package.name,
                        &self.request_channel.0,
                    ),
                ).await;
            }
        }
    }

    pub(crate) async fn process_request(&mut self, data: RequestData) {
        use RequestData::*;

        match data {
            CreateNode { package, schema } => { self.create_node(&package, &schema); }
            ConnectIO {
                output_node, output, input_node, input,
            } => { self.connect_io(output_node, &output, input_node, &input); }
            Event(event) => { self.handle_event(event).await; }
            Stop => {
                // self.graph.stop();
            }
            _ => {}
        };
    }

    pub fn create_node(&mut self, package: &str, schema: &str) -> Option<NodeRef> {
        let schema = self.package(&package).and_then(|p| p.schema(&schema));

        if let Some(schema) = schema {
            let schema = schema.clone();

            let node = self.graph.create_node(&schema);

            if let NodeSchema::Event(schema) = &*schema {
                let listeners = self.event_map
                    .entry((schema.package.clone(), schema.id.clone()))
                    .or_insert(vec![]);

                listeners.push(node.clone());
            }

            return Some(node);
        }

        None
    }

    pub fn connect_io(&mut self, output_node: i32, output: &str, input_node: i32, input: &str) -> Result<(), ConnectIOError> {
        let output_node = self
            .graph
            .node(output_node);

        let input_node = self
            .graph
            .node(input_node);

        let (output_node, input_node) = match (output_node, input_node) {
            (Some(output_node), Some(input_node)) => (output_node, input_node),
            (input, output) =>
                return Err(ConnectIOError::InvalidNodes {
                    input: input.is_none(),
                    output: output.is_none(),
                })
        };

        let (output, input) = match (output_node.find_output(output), input_node.find_input(input)) {
            (Some(output), Some(input)) => (output, input),
            (output, input) =>
                return Err(ConnectIOError::InvalidIO {
                    input: input.is_none(),
                    output: output.is_none(),
                })
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

    pub async fn start(&mut self) {
        loop {
            let event = self.request_channel.1.recv().await.unwrap();

            self.process_request(event.data).await;
        }
    }

    pub async fn handle_event(&self, event: Event) {
        let listeners = self.event_map.get(&(event.package.clone(), event.event.clone()));

        if let Some(listeners) = listeners {
            let futures: Vec<_> = listeners.iter()
                .map(|node| self.fire_node(node.id, &event.data)).collect();

            futures::future::join_all(futures).await;
        }
    }

    pub(crate) async fn execute_node(&self, node: NodeRef) -> ExecuteOutput {
        for input in node.inputs.lock().unwrap().iter() {
            if let Input::Data(input) = input {
                if let Some(output) = &*input.connected_output.lock().unwrap() {
                    match &*output.node.schema {
                        NodeSchema::Exec(_) | NodeSchema::Event(_) => input.set_value(output.get_value()),
                        _ => {}
                    };
                } else { input.reset_value(); }
            }
        }

        if let NodeSchema::Exec(schema) = &*node.schema {
            let engine = self.package(&schema.package).unwrap().engine.clone();

            let context = ExecuteContext {
                engine
            };

            match &schema.execute {
                ExecuteFn::Sync(func) => {
                    (func)(node.clone(), context)
                }
                ExecuteFn::Async(func) => {
                    (func)(node.clone(), context).await
                }
            }
        } else { None }
    }

    pub(crate) async fn fire_node(&self, node_id: i32, data: &Value) {
        let node = self.graph.nodes.get(&node_id).unwrap();

        let schema = match &*node.schema {
            NodeSchema::Event(schema) => schema,
            _ => return
        };

        let res = (schema.fire)(node.clone(), data.clone());

        let mut next_node_mut = res
            .and_then(|o|
                o.connected_input.lock().unwrap().clone()
            ).map(|i| i.node.clone());

        while let Some(next_node) = next_node_mut.clone() {
            let next_output = self.execute_node(next_node).await;

            next_node_mut = match next_output {
                Some(next_output) =>
                    next_output.connected_input
                        .lock().unwrap().as_ref()
                        .map(|i| i.node.clone()),
                None => None
            };
        }
    }
}

#[cfg(test)]
mod test {
    use crate::packages::load_packages;
    use crate::packages::keyboard::key::Key;
    use crate::core::Core;

    async fn setup_core() -> Core {
        let mut core = Core::new();

        load_packages(&mut core);

        core.start_engines().await;

        core
    }

    #[tokio::test]
    async fn creates_node() {
        let mut core = setup_core().await;

        core.create_node(
            "keyboard",
            "key_a",
        );

        assert_eq!(core.graph.nodes.len(), 1);
    }

    #[tokio::test]
    async fn connects_nodes() {
        let mut core = setup_core().await;

        let key_pressed_node = core.create_node(
            "keyboard",
            "key_a",
        ).unwrap();

        let print_node = core.create_node(
            "utils",
            "print",
        ).unwrap();

        let values_connected = core.connect_io(key_pressed_node.id, "key", print_node.id, "value");
        assert_eq!(values_connected, Ok(()));
    }

    use crate::packages::keyboard::key_event::KeyEvent;

    #[tokio::test]
    async fn executes_nodes() {
        let mut core = setup_core().await;

        let key_pressed_node = core.create_node(
            "keyboard",
            "key_a",
        ).unwrap();

        let print_node = core.create_node(
            "utils",
            "print",
        ).unwrap();

        core.connect_io(
            key_pressed_node.id,
            "key",
            print_node.id,
            "value",
        ).unwrap();
        core.connect_io(
            key_pressed_node.id,
            "execute",
            print_node.id,
            "execute",
        ).unwrap();

        core.fire_node(key_pressed_node.id, &KeyEvent {
            key: Key::A,
            shift_pressed: false,
            ctrl_pressed: false,
            alt_pressed: false,
            meta_pressed: false,
        }.to_value()).await;
    }
}
