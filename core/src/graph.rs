use crate::node::{Node, Position};
use crate::schema::NodeSchema;
use std::{collections::HashMap, sync::Arc};

pub struct Graph {
    pub id: i32,
    pub name: String,
    pub nodes: HashMap<i32, Arc<Node>>,
    id_counter: i32,
}

impl Graph {
    pub fn new(id: i32, name: String) -> Self {
        Self {
            id,
            name,
            id_counter: 0,
            nodes: HashMap::new(),
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn generate_id(&mut self) -> i32 {
        let id = self.id_counter;
        self.id_counter += 1;
        id
    }

    pub fn create_node(&mut self, schema: &Arc<NodeSchema>, position: Position) -> Arc<Node> {
        let id = self.generate_id();

        let node = Node::new(id, self.id, schema, position);

        self.nodes.insert(id, node.clone());

        node.clone()
    }

    pub fn delete_node(&mut self, node: i32) {
        self.nodes.remove(&node);
    }

    pub fn node(&self, id: i32) -> Option<&Arc<Node>> {
        self.nodes.get(&id)
    }

    pub fn reset(&mut self) {
        self.nodes.clear();
    }
}
