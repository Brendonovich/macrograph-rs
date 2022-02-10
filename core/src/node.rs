use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use macrograph_package_api::{
    value::types::{PrimitiveType, ValueType},
    BuildSchema, IOProxy, InputSchema, OutputSchema,
};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{io::*, schema::NodeSchema, value::Value};

#[derive(TS, Serialize, Deserialize, Debug, Clone)]
#[ts(export)]
pub struct Position {
    x: f64,
    y: f64,
}

pub struct Node {
    pub id: i32,
    pub graph_id: i32,
    pub position: Mutex<Position>,
    pub schema: Arc<NodeSchema>,
    // Structs referencing IO must not hold strong references
    // Dropping the node should also drop the IO, so only use Weaks
    pub inputs: Mutex<Vec<Input>>,
    pub outputs: Mutex<Vec<Output>>,
}

impl Node {
    pub fn new(id: i32, graph_id: i32, schema: &Arc<NodeSchema>, position: Position) -> Arc<Self> {
        let schema = schema.clone();

        let node = Arc::new(Self {
            id,
            graph_id,
            position: Mutex::new(position),
            schema: schema.clone(),
            inputs: Mutex::new(vec![]),
            outputs: Mutex::new(vec![]),
        });

        let mut ctx = BuildSchema::new();

        schema.build(&mut ctx);

        node.reconcile_io(ctx);

        node
    }

    pub fn set_position(&self, position: Position) {
        *self.position.lock().unwrap() = position;
    }

    fn reconcile_io(self: &Arc<Self>, ctx: BuildSchema) {
        let mut inputs = self.inputs.lock().unwrap();
        let inputs_count = ctx.inputs.len();

        for (index, schema) in ctx.inputs.into_iter().enumerate() {
            match schema {
                InputSchema::Exec(name) => match inputs.iter().position(|i| i.get_name() == name) {
                    Some(input_index) => {
                        let input = &inputs[input_index];

                        match input {
                            Input::Exec(_) => {
                                if input_index != index {
                                    inputs.swap(input_index, index);
                                }
                            }
                            _ => {}
                        }
                    }
                    None => inputs.insert(index, ExecInput::new(name, &self)),
                },
                InputSchema::Data(name, r#type) => {
                    match inputs.iter().position(|i| i.get_name() == name) {
                        Some(input_index) => {
                            let input = &inputs[input_index];

                            match input {
                                Input::Data(input) => {
                                    if input.r#type != r#type {
                                        input.disconnect();
                                    }

                                    if let ValueType::Primitive(t) = r#type {
                                        input.set_value(Arc::new(t.into()));
                                    }

                                    if input_index != index {
                                        inputs.swap(input_index, index);
                                    }
                                }
                                _ => {}
                            }
                        }
                        None => inputs.insert(index, DataInput::new(name, r#type, &self)),
                    }
                }
            }
        }

        inputs.drain(inputs_count..);

        let mut outputs = self.outputs.lock().unwrap();
        let outputs_count = ctx.outputs.len();

        for (index, schema) in ctx.outputs.into_iter().enumerate() {
            match schema {
                OutputSchema::Exec(name) => {
                    match outputs.iter().position(|i| i.get_name() == name) {
                        Some(output_index) => {
                            let output = &outputs[output_index];

                            match output {
                                Output::Exec(_) => {
                                    if output_index != index {
                                        outputs.swap(output_index, index);
                                    }
                                }
                                _ => {}
                            }
                        }
                        None => outputs.insert(index, ExecOutput::new(name, &self)),
                    }
                }
                OutputSchema::Data(name, r#type) => {
                    match outputs.iter().position(|i| i.get_name() == name) {
                        Some(output_index) => {
                            let output = &outputs[output_index];

                            match output {
                                Output::Data(output) => {
                                    if output.r#type != r#type {
                                        output.disconnect();
                                    }

                                    output.set_value(r#type.into());

                                    if output_index != index {
                                        outputs.swap(output_index, index);
                                    }
                                }
                                _ => {}
                            }
                        }
                        None => outputs
                            .insert(index, DataOutput::new(name, r#type, r#type.into(), &self)),
                    }
                }
            }
        }

        outputs.drain(outputs_count..);
    }

    pub fn find_input(&self, name: &str) -> Option<Input> {
        self.inputs
            .lock()
            .unwrap()
            .iter()
            .find(|i| i.get_name() == name)
            .map(|i| i.clone())
    }

    pub fn find_output(&self, name: &str) -> Option<Output> {
        self.outputs
            .lock()
            .unwrap()
            .iter()
            .find(|o| o.get_name() == name)
            .map(|o| o.clone())
    }

    pub fn find_data_input(&self, name: &str) -> Option<Arc<DataInput>> {
        self.find_input(name).and_then(|i| {
            if let Input::Data(i) = i {
                Some(i)
            } else {
                None
            }
        })
    }

    pub fn find_data_output(&self, name: &str) -> Option<Arc<DataOutput>> {
        self.find_output(name).and_then(|o| {
            if let Output::Data(o) = o {
                Some(o)
            } else {
                None
            }
        })
    }

    pub fn find_exec_input(&self, name: &str) -> Option<Arc<ExecInput>> {
        self.find_input(name).and_then(|i| {
            if let Input::Exec(i) = i {
                Some(i)
            } else {
                None
            }
        })
    }

    pub fn find_exec_output(&self, name: &str) -> Option<Arc<ExecOutput>> {
        self.find_output(name).and_then(|o| {
            if let Output::Exec(o) = o {
                Some(o)
            } else {
                None
            }
        })
    }

    pub fn get_io_data(&self) -> IOProxy {
        let mut inputs = HashMap::new();

        for input in self.inputs.lock().unwrap().iter() {
            match input {
                Input::Data(input) => {
                    inputs.insert(input.name.clone(), (**input.value.load()).clone());
                }
                _ => {}
            };
        }

        IOProxy {
            inputs,
            outputs: HashMap::new(),
        }
    }

    pub fn parse_io_data(self: &Arc<Self>, data: IOProxy) {
        let outputs = self.outputs.lock().unwrap();

        for (name, value) in data.outputs.into_iter() {
            outputs.iter().find(|o| o.get_name() == name).map(|o| {
                if let Output::Data(o) = o {
                    o.set_value(value);
                }
            });
        }
    }
}
