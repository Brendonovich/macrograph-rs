use serde::Serialize;
use ts_rs::TS;

use crate::node::{Node, Position};

use super::{
    io::{RawInput, RawOutput},
    schema::RawNodeSchemaRef,
};

#[derive(TS, Serialize, Debug)]
#[ts(export)]
#[serde(rename = "Node")]
pub struct RawNode {
    pub id: i32,
    #[ts(inline)]
    pub schema: RawNodeSchemaRef,
    pub position: Position,
    pub inputs: Vec<RawInput>,
    pub outputs: Vec<RawOutput>,
}

impl From<&Node> for RawNode {
    fn from(node: &Node) -> Self {
        Self {
            id: node.id,
            schema: node.schema.as_ref().into(),
            position: node.position.lock().unwrap().clone(),
            inputs: node.inputs.lock().unwrap().iter().map(|io| io.into()).collect(),
            outputs: node.outputs.lock().unwrap().iter().map(|io| io.into()).collect(),
        }
    }
}
