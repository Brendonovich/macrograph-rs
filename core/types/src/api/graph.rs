use serde::Serialize;
use ts_rs::TS;

use crate::graph::Graph;

use super::node::RawNode;

#[derive(TS, Serialize, Debug)]
#[ts(export)]
#[serde(rename = "Graph")]
pub struct RawGraph {
    pub id: i32,
    pub name: String,
    pub nodes: Vec<RawNode>,
}

impl From<&Graph> for RawGraph {
    fn from(graph: &Graph) -> Self {
        Self {
            id: graph.id,
            name: graph.name.clone(),
            nodes: graph.nodes.values().map(|n| n.as_ref().into()).collect(),
        }
    }
}
