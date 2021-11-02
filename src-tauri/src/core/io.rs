use std::sync::{Mutex, Arc};
use crate::core::value::Value;
use crate::core::types::NodeRef;

pub struct DataInput {
  pub name: String,
  default_value: Arc<Value>,
  pub value: Mutex<Arc<Value>>,
  pub connected_output: Mutex<Option<Arc<DataOutput>>>
}

impl DataInput {
  pub fn new(name: &str, default_value: Value) -> Self {
    let default_value = Arc::new(default_value);

    Self {
      name: name.to_string(),
      value: Mutex::new(default_value.clone()),
      default_value,
      connected_output: Mutex::new(None)
    }
  }

  pub fn get_value(&self) -> Arc<Value> {
    self.value.lock().unwrap().clone()
  }

  pub fn set_value(&self, value: Arc<Value>) {
    *self.value.lock().unwrap() = value.clone()
  }

  pub fn reset_value(&self) {
    self.set_value(self.default_value.clone())
  }

  pub fn connect_output(&self, output: &Arc<DataOutput>) {
    let mut input_connected_output = self.connected_output.lock().unwrap();

    if let Some(input_connected_output) = &*input_connected_output {
      let mut inputs = input_connected_output.connected_inputs.lock().unwrap();
      let index = inputs.iter().position(|i| std::ptr::eq( i.as_ref(), self)).unwrap();
      inputs.swap_remove(index);
    }

    *input_connected_output = Some(output.clone());
  }
}

pub struct ExecInput {
  pub name: String,
  pub node: NodeRef,
  connected_output: Mutex<Option<Arc<ExecOutput>>>
}

impl ExecInput {
  pub fn new(name: &str, node: &NodeRef) -> Self {
    Self {
      name: name.to_string(),
      node: node.clone(),
      connected_output: Mutex::new(None)
    }
  }

  pub fn connect_output(&self, output: &Arc<ExecOutput>) {
    *self.connected_output.lock().unwrap() = Some(output.clone());
  }
}

#[derive(Clone)]
pub enum Input {
  Data(Arc<DataInput>),
  Exec(Arc<ExecInput>)
}


impl Input {
  pub fn get_name(&self) -> &str {
    match self {
      Self::Exec(i) => &i.name,
      Self::Data(i) => &i.name
    }
  }
}

pub struct DataOutput {
  pub name: String,
  value: Mutex<Arc<Value>>,
  pub node: NodeRef,
  connected_inputs: Mutex<Vec<Arc<DataInput>>>
}

impl DataOutput {
  pub fn new(name: &str, value: Value, node: &NodeRef) -> Self {
    Self {
      name: name.to_string(),
      value: Mutex::new(Arc::new(value)),
      node: node.clone(),
      connected_inputs: Mutex::new(vec![])
    }
  }

  pub fn set_value(&self, value: Value) {
    *self.value.lock().unwrap() = Arc::new(value);
  }

  pub fn get_value(&self) -> Arc<Value> {
    self.value.lock().unwrap().clone()
  }

  pub fn connect_input(&self, input: &Arc<DataInput>) {
    self.connected_inputs.lock().unwrap().push(input.clone());
  }
}

pub struct ExecOutput {
  pub name: String,
  pub connected_input: Mutex<Option<Arc<ExecInput>>>
}

impl ExecOutput {
  pub fn new(name: &str) -> Self {
    Self {
      name: name.to_string(),
      connected_input: Mutex::new(None)
    }
  }

  pub fn connect_input(&self, input: &Arc<ExecInput>) {
    *self.connected_input.lock().unwrap() = Some(input.clone());
  }

  pub fn disconnect(&self) {
    *self.connected_input.lock().unwrap() = None;
  }
}

#[derive(Clone)]
pub enum Output {
  Data(Arc<DataOutput>),
  Exec(Arc<ExecOutput>)
}

impl Output {
  pub fn get_name(&self) -> &str {
    match self {
      Self::Exec(o) => &o.name,
      Self::Data(o) => &o.name
    }
  }
}