use serde::{Deserialize, Serialize};
use ts_rs::TS;

use self::{
    primitive::Primitive,
    types::{IntoType, ListType, PrimitiveType, ValueType},
};

pub mod list;
pub mod primitive;
pub mod types;

#[derive(TS, Serialize, Deserialize, Debug, Clone)]
#[ts(export)]
#[serde(rename_all = "camelCase", tag = "type", content = "value")]
pub enum Value {
    Primitive(primitive::Primitive),
    List(list::List),
}

impl Value {
    pub fn as_int(&self) -> Option<i32> {
        match self {
            Value::Primitive(primitive) => primitive.as_int(),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        match self {
            Value::Primitive(primitive) => primitive.as_float(),
            _ => None,
        }
    }

    pub fn as_string(&self) -> Option<String> {
        match self {
            Value::Primitive(primitive) => primitive.as_string(),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Primitive(primitive) => primitive.as_bool(),
            _ => None,
        }
    }

    pub fn as_list<T: IntoType<PrimitiveType>>(&self) -> Option<list::List> {
        match self {
            Value::List(list) => {
                if list.r#type == T::into_type().into() {
                    Some(list.clone())
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn same_type(left: &Self, right: &Self) -> bool {
        match (left, right) {
            (Self::Primitive(left), Self::Primitive(right)) => {
                primitive::Primitive::same_type(left, right)
            }
            (Self::List(left), Self::List(right)) => left.r#type == right.r#type,
            _ => false,
        }
    }

    pub fn r#type(&self) -> ValueType {
        match self {
            Value::Primitive(value) => ValueType::Primitive(value.r#type()),
            Value::List(array) => ValueType::List(array.r#type),
        }
    }
}

impl<T: Into<Primitive>> From<T> for Value {
    fn from(value: T) -> Self {
        Self::Primitive(value.into())
    }
}

impl From<ValueType> for Value {
    fn from(value: ValueType) -> Self {
        match value {
            ValueType::Primitive(primitive) => Self::Primitive(primitive.into()),
            ValueType::List(list) => Self::List(list.into()),
        }
    }
}
