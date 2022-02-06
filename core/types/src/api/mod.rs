use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{node::Position, Value};

use self::{
    graph::RawGraph,
    io::{RawInput, RawOutput},
    package::RawPackage,
};

pub mod graph;
pub mod io;
pub mod node;
pub mod package;
pub mod schema;

#[derive(TS, Serialize, Deserialize, Debug)]
#[ts(export)]
#[serde(tag = "type", content = "data")]
pub enum Request {
    CreateNode {
        graph: i32,
        package: String,
        schema: String,
        position: Position,
    },
    DeleteNode {
        graph: i32,
        node: i32,
    },
    SetDefaultValue {
        graph: i32,
        node: i32,
        input: String,
        value: Value,
    },
    ConnectIO {
        graph: i32,
        output_node: i32,
        output: String,
        input_node: i32,
        input: String,
    },
    DisconnectIO {
        graph: i32,
        node: i32,
        io: String,
        is_input: bool,
    },
    GetPackages,
    GetProject,
    Reset {
        graph: i32,
    },
}

#[derive(TS, Serialize, Debug)]
#[ts(export)]
#[serde(tag = "type", content = "data")]
pub enum Response {
    CreateNode {
        id: i32,
        inputs: Vec<RawInput>,
        outputs: Vec<RawOutput>,
    },
    DeleteNode,
    SetDefaultValue,
    ConnectIO,
    DisconnectIO,
    GetPackages {
        packages: Vec<RawPackage>,
    },
    GetProject {
        graphs: Vec<RawGraph>,
    },
    Reset,
}
