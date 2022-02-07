use std::collections::HashMap;

use crate::Value;

#[derive(Debug)]
pub struct IOProxy {
    pub inputs: HashMap<String, Value>,
    pub outputs: HashMap<String, Value>,
}

impl IOProxy {
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

    pub fn get_float(&self, name: &str) -> Option<f64> {
        self.inputs.get(name).and_then(|v| v.as_float())
    }

    pub fn set_float(&mut self, name: &str, value: f64) {
        self.outputs.insert(name.to_string(), value.into());
    }

    pub fn get_int(&self, name: &str) -> Option<i32> {
        self.inputs.get(name).and_then(|v| v.as_int())
    }

    pub fn set_int(&mut self, name: &str, value: i32) {
        self.outputs.insert(name.to_string(), value.into());
    }
}

pub enum InputSchema {
    Exec(String),
    Data(String, Value),
}

pub enum OutputSchema {
    Exec(String),
    Data(String, Value),
}

pub struct BuildSchema {
    pub inputs: Vec<InputSchema>,
    pub outputs: Vec<OutputSchema>,
}

impl BuildSchema {
    pub fn new() -> Self {
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
