use serde::{Serialize, Deserialize};
use ts_rs::TS;

#[derive(TS, Serialize, Deserialize, Debug, Clone)]
#[ts(export)]
#[serde(rename_all = "camelCase", tag = "type", content = "value")]
pub enum Value {
    // Int(i64),
    // Float(f64),
    String(String),
    Bool(bool),
}

impl Value {
    // pub fn as_int(&self) -> Option<i64> {
    //     match self {
    //         Value::Int(i) => Some(*i),
    //         _ => None,
    //     }
    // }

    // pub fn as_float(&self) -> Option<f64> {
    //     match self {
    //         Value::Float(f) => Some(*f),
    //         _ => None,
    //     }
    // }

    pub fn as_string(&self) -> Option<String> {
        match self {
            Value::String(s) => Some(s.clone()),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            // Self::Int(v) => write!(f, "{}", v),
            // Self::Float(v) => write!(f, "{}", v),
            Self::String(v) => write!(f, "{}", v),
            Self::Bool(v) => write!(f, "{}", v),
        }
    }
}

// impl From<i64> for Value {
//     fn from(value: i64) -> Value {
//         Value::Int(value)
//     }
// }

// impl From<f64> for Value {
//     fn from(value: f64) -> Value {
//         Value::Float(value)
//     }
// }

impl From<String> for Value {
    fn from(value: String) -> Value {
        Value::String(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Value {
        Value::String(value.to_string())
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Value {
        Value::Bool(value)
    }
}
