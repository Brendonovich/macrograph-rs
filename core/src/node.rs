use std::{
    hash::Hash,
    sync::{Arc, Mutex},
};

use serde::{Serialize, Deserialize};
use ts_rs::TS;

use crate::{io::*, types::*, Value};

#[derive(TS, Serialize, Deserialize, Debug)]
#[ts(export)]
pub struct Position {
    x: f64,
    y: f64,
}

pub struct Node {
    pub id: i32,
    pub name: String,
    pub position: Position,
    pub schema: NodeSchemaRef,
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

impl Node {
    pub(crate) fn new(id: i32, schema: &NodeSchemaRef, position: Position) -> NodeRef {
        let schema = schema.clone();

        let node = Arc::new(Self {
            id,
            position,
            name: schema.name.to_string(),
            schema: schema.clone(),
            inputs: Mutex::new(vec![]),
            outputs: Mutex::new(vec![]),
        });

        schema.build(node.clone());

        node
    }

    pub fn dispose(&self) {
        let mut outputs = self.outputs.lock().unwrap();
        let mut inputs = self.inputs.lock().unwrap();

        for output in outputs.iter_mut() {
            output.disconnect();
        }

        outputs.clear();
        inputs.clear();
    }

    pub fn find_input(&self, name: &str) -> Option<Input> {
        self.inputs
            .lock()
            .unwrap()
            .iter()
            .find(|i| i.get_id() == name)
            .map(|i| i.clone())
    }

    pub fn find_output(&self, name: &str) -> Option<Output> {
        self.outputs
            .lock()
            .unwrap()
            .iter()
            .find(|o| o.get_id() == name)
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

    pub fn add_data_input(&self, input: DataInput) {
        self.inputs
            .lock()
            .unwrap()
            .push(Input::Data(Arc::new(input)));
    }

    pub fn add_exec_input(&self, input: ExecInput) {
        self.inputs
            .lock()
            .unwrap()
            .push(Input::Exec(Arc::new(input)));
    }

    pub fn add_data_output(&self, output: DataOutput) {
        self.outputs
            .lock()
            .unwrap()
            .push(Output::Data(Arc::new(output)));
    }

    pub fn add_exec_output(&self, output: ExecOutput) {
        self.outputs
            .lock()
            .unwrap()
            .push(Output::Exec(Arc::new(output)));
    }

    /* Value Getters */
    pub fn get_bool(&self, input: &str) -> Option<bool> {
        self.find_data_input(input)
            .and_then(|o| o.get_value().as_bool())
    }

    // pub fn get_int(&self, input: &str) -> Option<i64> {
    //     self.find_data_input(input)
    //         .and_then(|o| o.get_value().as_int())
    // }

    // pub fn get_float(&self, input: &str) -> Option<f64> {
    //     self.find_data_input(input)
    //         .and_then(|o| o.get_value().as_float())
    // }

    pub fn get_string(&self, input: &str) -> Option<String> {
        self.find_data_input(input)
            .and_then(|i| i.get_value().as_string())
    }

    pub fn set_output(&self, output: &str, value: Value) {
        self.find_data_output(output).map(|o| o.set_value(value));
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("dropping node")
    }
}
