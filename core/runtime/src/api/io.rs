use std::sync::Arc;

use serde::Serialize;
use ts_rs::TS;

use crate::{
    io::{Input, Output},
    Value,
};

#[derive(TS, Serialize, Debug)]
#[ts(export)]
#[serde(tag = "variant", rename = "Input")]
pub enum RawInput {
    Data {
        name: String,
        default_value: Arc<Value>,
    },
    Exec {
        name: String,
    },
}

impl From<&Input> for RawInput {
    fn from(input: &Input) -> Self {
        match input {
            Input::Data(input) => RawInput::Data {
                name: input.name.clone(),
                default_value: input.default_value.load().clone(),
            },
            Input::Exec(input) => RawInput::Exec {
                name: input.name.clone(),
            },
        }
    }
}

#[derive(TS, Serialize, Debug)]
#[ts(export)]
#[serde(tag = "variant", rename = "Output")]
pub enum RawOutput {
    Data {
        name: String,
        #[serde(rename = "type")]
        typ: Value,
    },
    Exec {
        name: String,
    },
}

impl From<&Output> for RawOutput {
    fn from(output: &Output) -> Self {
        match output {
            Output::Data(data_output) => RawOutput::Data {
                name: data_output.name.clone(),
                typ: (*data_output.get_value()).clone(),
            },
            Output::Exec(exec_output) => RawOutput::Exec {
                name: exec_output.name.clone(),
            },
        }
    }
}
