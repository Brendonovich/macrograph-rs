use arc_swap::{access::Access, ArcSwap};
use macrograph_package_api::{
    primitive::Primitive,
    value::types::{PrimitiveType, ValueType},
};
use weak_table::PtrWeakHashSet;

use crate::{node::Node, value::Value};
use std::sync::{Arc, Mutex, Weak};

pub struct DataInput {
    pub name: String,
    pub r#type: ValueType,
    pub value: ArcSwap<Value>,
    pub default_value: ArcSwap<Primitive>,
    pub node: Weak<Node>,
    pub connected_output: Mutex<Weak<DataOutput>>,
}

impl DataInput {
    pub fn new(name: String, r#type: ValueType, node: &Arc<Node>) -> Input {
        let value = Arc::new(match r#type {
            ValueType::Primitive(primitive) => primitive.into(),
            ValueType::List(list_type) => Primitive::Bool(false),
        });

        Input::Data(Arc::new(Self {
            name,
            r#type,
            value: ArcSwap::from(Arc::new(r#type.into())),
            default_value: ArcSwap::from(value),
            node: Arc::downgrade(node),
            connected_output: Mutex::new(Weak::new()),
        }))
    }

    pub fn get_value(&self) -> Arc<Value> {
        self.value.load().clone()
    }

    pub fn set_value(&self, value: Arc<Value>) {
        self.value.swap(value);
    }

    pub fn set_default_value(&self, value: Primitive) {
        if let ValueType::Primitive(r#type) = self.r#type {
            if r#type == value.r#type() {
                self.default_value.swap(Arc::new(value));
            }
        }
    }

    pub fn reset_value(&self) {
        match self.r#type {
            ValueType::Primitive(_) => self
                .value
                .swap(Arc::new(self.default_value.load().as_ref().clone().into())),
            ValueType::List(_) => Arc::new(self.r#type.into()),
        };
    }

    pub fn connect_output(&self, output: &Arc<DataOutput>) {
        self.disconnect();

        *self.connected_output.lock().unwrap() = Arc::downgrade(output);
    }

    pub fn disconnect(&self) {
        let mut input_connected_output = self.connected_output.lock().unwrap();

        if let Some(input_connected_output) = input_connected_output.upgrade() {
            let mut inputs = input_connected_output.connected_inputs.lock().unwrap();
            let input = inputs
                .iter()
                .find(|i| std::ptr::eq(i.as_ref(), self))
                .unwrap();
            inputs.remove(&input);
        }

        *input_connected_output = Weak::new();
    }
}

pub struct ExecInput {
    pub name: String,
    pub node: Weak<Node>,
    pub connected_output: Mutex<Weak<ExecOutput>>,
}

impl ExecInput {
    pub fn new(name: String, node: &Arc<Node>) -> Input {
        Input::Exec(Arc::new(Self {
            name,
            node: Arc::downgrade(node),
            connected_output: Mutex::new(Weak::new()),
        }))
    }

    pub fn connect_output(&self, output: &Arc<ExecOutput>) {
        self.disconnect();

        *self.connected_output.lock().unwrap() = Arc::downgrade(output)
    }

    pub fn disconnect(&self) {
        let mut connected_output = self.connected_output.lock().unwrap();

        if let Some(output) = connected_output.upgrade() {
            *output.connected_input.lock().unwrap() = Weak::new();
        }

        *connected_output = Weak::new();
    }
}

#[derive(Clone)]
pub enum Input {
    Data(Arc<DataInput>),
    Exec(Arc<ExecInput>),
}

impl Input {
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
    pub name: String,
    pub r#type: ValueType,
    pub value: ArcSwap<Value>,
    pub node: Weak<Node>,
    pub connected_inputs: Mutex<PtrWeakHashSet<Weak<DataInput>>>,
}

impl DataOutput {
    pub fn new(name: String, r#type: ValueType, value: Value, node: &Arc<Node>) -> Output {
        Output::Data(Arc::new(Self {
            name,
            r#type,
            value: ArcSwap::from_pointee(value),
            node: Arc::downgrade(node),
            connected_inputs: Mutex::new(PtrWeakHashSet::new()),
        }))
    }

    pub fn set_value(&self, value: Value) {
        self.value.swap(Arc::new(value));
    }

    pub fn connect_input(&self, input: &Arc<DataInput>) {
        self.connected_inputs.lock().unwrap().insert(input.clone());
    }

    pub fn disconnect(&self) {
        let mut connected_inputs = self.connected_inputs.lock().unwrap();
        connected_inputs
            .iter()
            .for_each(|i| *i.connected_output.lock().unwrap() = Weak::new());
        connected_inputs.clear();
    }
}

pub struct ExecOutput {
    pub name: String,
    pub node: Weak<Node>,
    pub connected_input: Mutex<Weak<ExecInput>>,
}

impl ExecOutput {
    pub fn new(name: String, node: &Arc<Node>) -> Output {
        Output::Exec(Arc::new(Self {
            name,
            node: Arc::downgrade(node),
            connected_input: Mutex::new(Weak::new()),
        }))
    }

    pub fn connect_input(&self, input: &Arc<ExecInput>) {
        self.disconnect();

        *self.connected_input.lock().unwrap() = Arc::downgrade(input);
    }

    pub fn disconnect(&self) {
        let mut connected_input = self.connected_input.lock().unwrap();

        if let Some(input) = connected_input.upgrade() {
            *input.connected_output.lock().unwrap() = Weak::new();
        }

        *connected_input = Weak::new();
    }
}

#[derive(Clone)]
pub enum Output {
    Data(Arc<DataOutput>),
    Exec(Arc<ExecOutput>),
}

impl Output {
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
