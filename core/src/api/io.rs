use macrograph_package_api::{primitive::Primitive, value::types::ValueType};
use serde::Serialize;
use ts_rs::TS;

use crate::io::{Input, Output};

#[derive(TS, Serialize, Debug)]
#[ts(export)]
pub struct Connection {
    node: i32,
    io: String,
}

#[derive(TS, Serialize, Debug)]
#[ts(export)]
#[serde(tag = "variant", rename = "Input")]
pub enum RawInput {
    Data {
        name: String,
        r#type: ValueType,
        default_value: Primitive,
        connection: Option<Connection>,
    },
    Exec {
        name: String,
        connection: Option<Connection>,
    },
}

impl From<&Input> for RawInput {
    fn from(input: &Input) -> Self {
        match input {
            Input::Data(input) => RawInput::Data {
                name: input.name.clone(),
                r#type: input.r#type.clone(),
                default_value: input.default_value.load().as_ref().clone(),
                connection: input
                    .connected_output
                    .lock()
                    .ok()
                    .and_then(|output| output.upgrade())
                    .map(|output| Connection {
                        node: output.node.upgrade().unwrap().id,
                        io: output.name.clone(),
                    }),
            },
            Input::Exec(input) => RawInput::Exec {
                name: input.name.clone(),
                connection: input
                    .connected_output
                    .lock()
                    .ok()
                    .and_then(|output| output.upgrade())
                    .map(|output| Connection {
                        node: output.node.upgrade().unwrap().id,
                        io: output.name.clone(),
                    }),
            },
        }
    }
}

#[derive(TS, Serialize, Debug)]
#[ts(export)]
#[serde(tag = "variant", rename = "Output")]
pub enum RawOutput {
    Data { name: String, r#type: ValueType },
    Exec { name: String },
}

impl From<&Output> for RawOutput {
    fn from(output: &Output) -> Self {
        match output {
            Output::Data(output) => RawOutput::Data {
                name: output.name.clone(),
                r#type: (**output.value.load()).r#type(),
            },
            Output::Exec(output) => RawOutput::Exec {
                name: output.name.clone(),
            },
        }
    }
}
