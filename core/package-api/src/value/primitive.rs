use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::types::PrimitiveType;

#[derive(TS, Serialize, Deserialize, Debug, Clone)]
#[ts(export)]
#[serde(rename_all = "camelCase", tag = "type", content = "value")]
pub enum Primitive {
    Int(i32),
    Float(f64),
    String(String),
    Bool(bool),
}

impl Primitive {
    pub fn as_int(&self) -> Option<i32> {
        match self {
            Primitive::Int(i) => Some(*i),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        match self {
            Primitive::Float(f) => Some(*f),
            _ => None,
        }
    }

    pub fn as_string(&self) -> Option<String> {
        match self {
            Primitive::String(s) => Some(s.clone()),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Primitive::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn same_type(left: &Self, right: &Self) -> bool {
        std::mem::discriminant(left) == std::mem::discriminant(right)
    }

    pub fn r#type(&self) -> PrimitiveType {
        match self {
            Primitive::Int(_) => PrimitiveType::Int,
            Primitive::Float(_) => PrimitiveType::Float,
            Primitive::String(_) => PrimitiveType::String,
            Primitive::Bool(_) => PrimitiveType::Bool,
        }
    }
}

impl std::fmt::Display for Primitive {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Int(v) => write!(f, "{}", v),
            Self::Float(v) => write!(f, "{}", v),
            Self::String(v) => write!(f, "{}", v),
            Self::Bool(v) => write!(f, "{}", v),
        }
    }
}

impl From<i32> for Primitive {
    fn from(value: i32) -> Primitive {
        Primitive::Int(value)
    }
}

impl From<f64> for Primitive {
    fn from(value: f64) -> Primitive {
        Primitive::Float(value)
    }
}

impl From<String> for Primitive {
    fn from(value: String) -> Primitive {
        Primitive::String(value)
    }
}

impl From<&String> for Primitive {
    fn from(value: &String) -> Primitive {
        Primitive::String(value.into())
    }
}

impl From<&str> for Primitive {
    fn from(value: &str) -> Primitive {
        Primitive::String(value.into())
    }
}

impl From<bool> for Primitive {
    fn from(value: bool) -> Primitive {
        Primitive::Bool(value)
    }
}

impl From<PrimitiveType> for Primitive {
    fn from(value: PrimitiveType) -> Primitive {
        match value {
            PrimitiveType::Int => Primitive::Int(0),
            PrimitiveType::Float => Primitive::Float(0.0),
            PrimitiveType::String => Primitive::String(String::new()),
            PrimitiveType::Bool => Primitive::Bool(false),
        }
    }
}
