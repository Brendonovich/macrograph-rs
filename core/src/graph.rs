use std::{collections::HashMap};

use crate::node::Position;

use super::{node::Node, types::*};

pub struct Graph {
    id_counter: i32,
    pub nodes: HashMap<i32, NodeRef>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            id_counter: 0,
            nodes: HashMap::new(),
        }
    }

    fn generate_id(&mut self) -> i32 {
        let id = self.id_counter;
        self.id_counter += 1;
        id
    }

    pub fn create_node(&mut self, schema: &NodeSchemaRef, position: Position) -> NodeRef {
        let id = self.generate_id();

        let node = Node::new(id, schema, position);

        self.nodes.insert(id, node.clone());

        node.clone()
    }

    pub fn node(&self, id: i32) -> Option<&NodeRef> {
        self.nodes.get(&id)
    }

    pub fn reset(&mut self) {
        for node in self.nodes.values_mut() {
            node.dispose();
        }

        self.nodes.clear();
    }

    // pub async fn execute_node(&self, id: i32) -> Option<i32> {
    //   let node = self.node(id);
    //
    //   if let Some(node_arc) = node {
    //     let schema = node_arc.schema.clone();
    //
    //     match &*schema {
    //       NodeSchema::Exec(schema) => match schema.execute {
    //         ExecuteFn::Sync(execute) => Some(execute(node_arc.clone())),
    //         ExecuteFn::Async(execute) => Some(execute(node_arc.clone()).await),
    //       },
    //       _ => todo!(),
    //     }
    //   } else {
    //     None
    //   }
    // }

    // pub async fn execute_connection(&self, output: OutputRef) {
    //   let
    // }
}
