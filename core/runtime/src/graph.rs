use macrograph_core_types::node::{Node, Position};
use macrograph_core_types::schema::NodeSchema;
use macrograph_core_types::*;
use std::{collections::HashMap, sync::Arc};

pub struct Graph {
    id_counter: i32,
    pub(crate) nodes: HashMap<i32, NodeRef>,
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

    pub(crate) fn create_node(&mut self, schema: &Arc<NodeSchema>, position: Position) -> NodeRef {
        let id = self.generate_id();

        let node = Node::new(id, schema, position);

        self.nodes.insert(id, node.clone());

        node.clone()
    }

    pub(crate) fn delete_node(&mut self, node: i32) {
        {
            let node = self.nodes.get(&node).unwrap();

            node.inputs
                .lock()
                .unwrap()
                .iter()
                .for_each(|i| i.disconnect());
            node.outputs
                .lock()
                .unwrap()
                .iter()
                .for_each(|i| i.disconnect());
        }
        
        self.nodes.remove(&node);
    }

    pub(crate) fn node(&self, id: i32) -> Option<&NodeRef> {
        self.nodes.get(&id)
    }

    pub fn reset(&mut self) {
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
