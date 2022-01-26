use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{node::Position, Value};

use self::{
    io::{RawInput, RawOutput},
    package::RawPackage,
};

pub mod io;
pub mod package;
pub mod schema;

#[derive(TS, Serialize, Deserialize, Debug)]
#[ts(export)]
#[serde(tag = "type", content = "data")]
pub enum Request {
    CreateNode {
        package: String,
        schema: String,
        position: Position,
    },
    SetDefaultValue {
        node: i32,
        input: String,
        value: Value,
    },
    ConnectIO {
        output_node: i32,
        output: String,
        input_node: i32,
        input: String,
    },
    DisconnectIO {
        node: i32,
        id: String,
        is_input: bool,
    },
    GetPackages,
    Reset,
}

#[derive(TS, Serialize, Debug)]
#[ts(export)]
#[serde(tag = "type", content = "data")]
pub enum Response {
    CreateNode {
        id: i32,
        name: String,
        inputs: Vec<RawInput>,
        outputs: Vec<RawOutput>,
    },
    SetDefaultValue,
    ConnectIO,
    DisconnectIO,
    GetPackages {
        packages: Vec<RawPackage>,
    },
    Reset,
}
