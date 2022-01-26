use arc_swap::ArcSwap;

use crate::types::NodeRef;
use crate::value::Value;
use std::sync::{Arc};
use no_deadlocks::Mutex;

pub struct DataInput {
    pub id: String,
    pub name: String,
    pub default_value: ArcSwap<Value>,
    pub value: ArcSwap<Value>,
    pub connected_output: Mutex<Option<Arc<DataOutput>>>,
}

impl DataInput {
    pub fn new(id: &str, name: &str, default_value: Value) -> Self {
        let value = Arc::new(default_value);

        Self {
            id: id.into(),
            name: name.into(),
            value: ArcSwap::from(value.clone()),
            default_value: ArcSwap::from(value),
            connected_output: Mutex::new(None),
        }
    }

    pub fn get_value(&self) -> Arc<Value> {
        self.value.load().clone()
    }

    pub fn set_value(&self, value: Arc<Value>) {
        self.value.swap(value);
    }

    pub fn set_default_value(&self, value: Value) {
        println!("setting default value: {:?}", value);
        if std::mem::discriminant(self.default_value.load().as_ref())
            == std::mem::discriminant(&value)
        {
            self.default_value.swap(Arc::new(value));
        }
    }

    pub fn reset_value(&self) {
        self.set_value(self.default_value.load().clone())
    }

    pub fn connect_output(&self, output: &Arc<DataOutput>) {
        self.disconnect();

        *self.connected_output.lock().unwrap() = Some(output.clone());
    }

    pub fn disconnect(&self) {
        let mut input_connected_output = self.connected_output.lock().unwrap();

        if let Some(input_connected_output) = &*input_connected_output {
            let mut inputs = input_connected_output.connected_inputs.lock().unwrap();
            let index = inputs
                .iter()
                .position(|i| std::ptr::eq(i.as_ref(), self))
                .unwrap();
            inputs.swap_remove(index);
        }

        *input_connected_output = None;
    }
}

pub struct ExecInput {
    pub id: String,
    pub name: String,
    pub node: NodeRef,
    pub connected_output: Mutex<Option<Arc<ExecOutput>>>,
}

impl ExecInput {
    pub fn new(id: &str, name: &str, node: &NodeRef) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            node: node.clone(),
            connected_output: Mutex::new(None),
        }
    }

    pub fn connect_output(&self, output: &Arc<ExecOutput>) {
        self.disconnect();

        *self.connected_output.lock().unwrap() = Some(output.clone());
    }

    pub fn disconnect(&self) {
        let mut connected_output = self.connected_output.lock().unwrap();

        if let Some(output) = &*connected_output {
            *output.connected_input.lock().unwrap() = None;
        }

        *connected_output = None;
    }
}

#[derive(Clone)]
pub enum Input {
    Data(Arc<DataInput>),
    Exec(Arc<ExecInput>),
}

impl Input {
    pub fn get_id(&self) -> &str {
        match self {
            Self::Exec(o) => &o.id,
            Self::Data(o) => &o.id,
        }
    }

    pub fn get_name(&self) -> &str {
        match self {
            Self::Exec(o) => &o.name,
            Self::Data(o) => &o.name,
        }
    }
    
    pub fn disconnect(&self) {
        match self {
            Self::Exec(o) => o.disconnect(),
            Self::Data(o) => o.disconnect(),
        }
    }
}

pub struct DataOutput {
    pub id: String,
    pub name: String,
    value: ArcSwap<Value>,
    pub node: NodeRef,
    pub connected_inputs: Mutex<Vec<Arc<DataInput>>>,
}

impl DataOutput {
    pub fn new(id: &str, name: &str, value: Value, node: &NodeRef) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            value: ArcSwap::from_pointee(value),
            node: node.clone(),
            connected_inputs: Mutex::new(vec![]),
        }
    }

    pub fn set_value(&self, value: Value) {
        self.value.swap(Arc::new(value));
    }

    pub fn get_value(&self) -> Arc<Value> {
        self.value.load().clone()
    }

    pub fn connect_input(&self, input: &Arc<DataInput>) {
        self.connected_inputs.lock().unwrap().push(input.clone());
    }

    pub fn disconnect(&self) {
        let mut connected_inputs = self.connected_inputs.lock().unwrap();
        connected_inputs
            .iter()
            .for_each(|i| *i.connected_output.lock().unwrap() = None);
        connected_inputs.clear();
    }
}

pub struct ExecOutput {
    pub id: String,
    pub name: String,
    pub connected_input: Mutex<Option<Arc<ExecInput>>>,
}

impl ExecOutput {
    pub fn new(id: &str, name: &str) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            connected_input: Mutex::new(None),
        }
    }

    pub fn connect_input(&self, input: &Arc<ExecInput>) {
        self.disconnect();

        *self.connected_input.lock().unwrap() = Some(input.clone());
    }

    pub fn disconnect(&self) {
        let mut connected_input = self.connected_input.lock().unwrap();

        if let Some(input) = &*connected_input {
            *input.connected_output.lock().unwrap() = None;
        }
        
        *connected_input = None;
    }
}

#[derive(Clone)]
pub enum Output {
    Data(Arc<DataOutput>),
    Exec(Arc<ExecOutput>),
}

impl Output {
    pub fn get_id(&self) -> &str {
        match self {
            Self::Exec(o) => &o.id,
            Self::Data(o) => &o.id,
        }
    }

    pub fn get_name(&self) -> &str {
        match self {
            Self::Exec(o) => &o.name,
            Self::Data(o) => &o.name,
        }
    }

    pub fn disconnect(&self) {
        match self {
            Self::Exec(o) => o.disconnect(),
            Self::Data(o) => o.disconnect(),
        }
    }
}
