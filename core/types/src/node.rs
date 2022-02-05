use std::{
    collections::HashMap,
    hash::Hash,
    sync::{Arc, Mutex},
};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{io::*, schema::NodeSchema, types::*, Value};

#[derive(TS, Serialize, Deserialize, Debug)]
#[ts(export)]
pub struct Position {
    x: f64,
    y: f64,
}

pub struct Node {
    pub id: i32,
    pub position: Position,
    pub schema: Arc<NodeSchema>,
    // IO is owned by the node, and since IO holds a weak reference to the node
    // everything will drop normally.
    pub inputs: Mutex<Vec<Input>>,
    pub outputs: Mutex<Vec<Output>>,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Node {}

impl Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

enum InputSchema {
    Exec(String),
    Data(String, Value),
}

enum OutputSchema {
    Exec(String),
    Data(String, Value),
}

pub struct BuildSchema {
    inputs: Vec<InputSchema>,
    outputs: Vec<OutputSchema>,
}

#[derive(Debug)]
pub struct IOData {
    inputs: HashMap<String, Value>,
    outputs: HashMap<String, Value>,
}

impl IOData {
    pub fn get_bool(&self, name: &str) -> Option<bool> {
        self.inputs.get(name).and_then(|v| v.as_bool())
    }

    pub fn set_string(&mut self, name: &str, value: &str) {
        self.outputs.insert(name.to_string(), value.into());
    }

    pub fn get_string(&self, name: &str) -> Option<String> {
        self.inputs.get(name).and_then(|v| v.as_string())
    }

    pub fn set_bool(&mut self, name: &str, value: bool) {
        self.outputs.insert(name.to_string(), value.into());
    }
}

impl BuildSchema {
    fn new() -> Self {
        Self {
            inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }

    pub fn exec_input(&mut self, name: &str) {
        self.inputs.push(InputSchema::Exec(name.into()));
    }

    pub fn exec_output(&mut self, name: &str) {
        self.outputs.push(OutputSchema::Exec(name.into()));
    }

    pub fn data_input(&mut self, name: &str, default_value: Value) {
        self.inputs
            .push(InputSchema::Data(name.into(), default_value));
    }

    pub fn data_output(&mut self, name: &str, default_value: Value) {
        self.outputs
            .push(OutputSchema::Data(name.into(), default_value));
    }
}

impl Node {
    pub fn new(id: i32, schema: &Arc<NodeSchema>, position: Position) -> NodeRef {
        let schema = schema.clone();

        let node = Arc::new(Self {
            id,
            position,
            schema: schema.clone(),
            inputs: Mutex::new(vec![]),
            outputs: Mutex::new(vec![]),
        });

        let mut ctx = BuildSchema::new();

        schema.build(&mut ctx);

        node.reconcile_io(ctx);

        node
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
                    None => {
                        inputs.insert(index, Input::Exec(Arc::new(ExecInput::new(name, &self))))
                    }
                },
                InputSchema::Data(name, default_value) => {
                    match inputs.iter().position(|i| i.get_name() == name) {
                        Some(input_index) => {
                            let input = &inputs[input_index];

                            match input {
                                Input::Data(input) => {
                                    if !Value::is_same_type(
                                        &input.default_value.load(),
                                        &default_value,
                                    ) {
                                        input.disconnect();
                                    }

                                    input.set_default_value(default_value);

                                    if input_index != index {
                                        inputs.swap(input_index, index);
                                    }
                                }
                                _ => {}
                            }
                        }
                        None => inputs.insert(
                            index,
                            Input::Data(Arc::new(DataInput::new(name, default_value))),
                        ),
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
                        None => {
                            outputs.insert(index, Output::Exec(Arc::new(ExecOutput::new(name))))
                        }
                    }
                }
                OutputSchema::Data(name, default_value) => {
                    match outputs.iter().position(|i| i.get_name() == name) {
                        Some(output_index) => {
                            let output = &outputs[output_index];

                            match output {
                                Output::Data(output) => {
                                    if !Value::is_same_type(&output.value.load(), &default_value) {
                                        output.disconnect();
                                    }

                                    output.set_value(default_value);

                                    if output_index != index {
                                        outputs.swap(output_index, index);
                                    }
                                }
                                _ => {}
                            }
                        }
                        None => outputs.insert(
                            index,
                            Output::Data(Arc::new(DataOutput::new(name, default_value, &self))),
                        ),
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

    pub fn get_io_data(&self) -> IOData {
        let mut inputs = HashMap::new();

        for input in self.inputs.lock().unwrap().iter() {
            match input {
                Input::Data(input) => {
                    inputs.insert(input.name.clone(), (**input.value.load()).clone());
                }
                _ => {}
            };
        }

        IOData {
            inputs,
            outputs: HashMap::new(),
        }
    }

    pub fn parse_io_data(self: &Arc<Self>, data: IOData) {
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
