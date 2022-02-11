use std::collections::HashMap;

use crate::{
    list::List,
    primitive::Primitive,
    value::types::{IntoType, ListType, PrimitiveType, ValueType},
    Value,
};

#[derive(Debug)]
pub struct IOProxy {
    pub inputs: HashMap<String, Value>,
    pub outputs: HashMap<String, Value>,
}

impl IOProxy {
    #[inline]
    pub fn get_bool(&self, name: &str) -> Option<bool> {
        self.inputs.get(name).and_then(|v| v.as_bool())
    }

    #[inline]
    pub fn set_string(&mut self, name: &str, value: String) {
        self.outputs.insert(name.to_string(), value.into());
    }

    #[inline]
    pub fn get_string(&self, name: &str) -> Option<String> {
        self.inputs.get(name).and_then(|v| v.as_string())
    }

    #[inline]
    pub fn set_bool(&mut self, name: &str, value: bool) {
        self.outputs.insert(name.to_string(), value.into());
    }

    #[inline]
    pub fn get_float(&self, name: &str) -> Option<f64> {
        self.inputs.get(name).and_then(|v| v.as_float())
    }

    #[inline]
    pub fn set_float(&mut self, name: &str, value: f64) {
        self.outputs.insert(name.to_string(), value.into());
    }

    #[inline]
    pub fn get_int(&self, name: &str) -> Option<i32> {
        self.inputs.get(name).and_then(|v| v.as_int())
    }

    #[inline]
    pub fn set_int(&mut self, name: &str, value: i32) {
        self.outputs.insert(name.to_string(), value.into());
    }

    #[inline]
    pub fn get_list<T: IntoType<PrimitiveType>>(&self, name: &str) -> Option<List> {
        self.inputs.get(name).and_then(|v| v.as_list::<T>())
    }

    pub fn set_list<T: Into<Primitive> + IntoType<PrimitiveType>>(
        &mut self,
        name: &str,
        value: Vec<T>,
    ) {
        self.outputs
            .insert(name.to_string(), Value::List(value.into()));
    }
}

pub enum InputSchema {
    Exec(String),
    Data(String, ValueType),
}

pub enum OutputSchema {
    Exec(String),
    Data(String, ValueType),
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

    #[inline]
    pub fn exec_input(&mut self, name: &str) {
        self.inputs.push(InputSchema::Exec(name.into()));
    }

    #[inline]
    pub fn exec_output(&mut self, name: &str) {
        self.outputs.push(OutputSchema::Exec(name.into()));
    }

    #[inline]
    pub fn data_input(&mut self, name: &str, r#type: ValueType) {
        self.inputs.push(InputSchema::Data(name.into(), r#type));
    }

    #[inline]
    pub fn int_input(&mut self, name: &str) {
        self.data_input(name, PrimitiveType::Int.into());
    }

    #[inline]
    pub fn float_input(&mut self, name: &str) {
        self.data_input(name, PrimitiveType::Float.into());
    }

    #[inline]
    pub fn bool_input(&mut self, name: &str) {
        self.data_input(name, PrimitiveType::Bool.into());
    }

    #[inline]
    pub fn string_input(&mut self, name: &str) {
        self.data_input(name, PrimitiveType::String.into());
    }

    #[inline]
    pub fn list_input<T: IntoType<PrimitiveType>>(&mut self, name: &str) {
        self.data_input(name, ListType::Primitive(T::into_type()).into());
    }

    #[inline]
    pub fn data_output(&mut self, name: &str, r#type: ValueType) {
        self.outputs.push(OutputSchema::Data(name.into(), r#type));
    }

    #[inline]
    pub fn int_output(&mut self, name: &str) {
        self.data_output(name, PrimitiveType::Int.into());
    }

    #[inline]
    pub fn float_output(&mut self, name: &str) {
        self.data_output(name, PrimitiveType::Float.into());
    }

    #[inline]
    pub fn bool_output(&mut self, name: &str) {
        self.data_output(name, PrimitiveType::Bool.into());
    }

    #[inline]
    pub fn string_output(&mut self, name: &str) {
        self.data_output(name, PrimitiveType::String.into());
    }

    #[inline]
    pub fn list_output<T: IntoType<PrimitiveType>>(&mut self, name: &str) {
        self.data_output(name, ListType::Primitive(T::into_type()).into());
    }
}
