use std::collections::HashMap;

use super::{
  node::Node,
  types::*,
};

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

  pub fn create_node(&mut self, schema: &NodeSchemaRef) -> NodeRef {
    let id = self.generate_id();

    let node = Node::new(id, schema);

    self.nodes.insert(id, node.clone());

    node.clone()
  }

  pub fn node(&self, id: i32) -> Option<&NodeRef> {
    self.nodes.get(&id)
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

// #[cfg(test)]
// mod test {
//   use std::time::Duration;
//
//   use crate::{
//     core::{schema::NodeSchema},
//     exec_fn,
//   };
//
//   use super::Graph;
//
//   #[test]
//   fn creates_node() {
//     let mut graph = Graph::new();
//
//     let schema = NodeSchema::new_exec("test", |_n| {}, exec_fn!(|_n, _c| { 1 }));
//     let node = graph.create_node(&schema);
//
//     assert_eq!(node.id, graph.node(0).unwrap().id)
//   }
//
//   #[tokio::test]
//   async fn executes_sync_node() {
//     let mut graph = Graph::new();
//
//     let schema = NodeSchema::new_exec("test", |_n| {}, exec_fn!(|_n, _c| { 69 }));
//     graph.create_node(&schema);
//
//     assert_eq!(graph.execute_node(0).await.unwrap(), 69);
//   }
//
//   #[tokio::test]
//   async fn executes_async_node() {
//     let mut graph = Graph::new();
//
//     let schema = NodeSchema::new_exec(
//       "test",
//       |_n| {},
//       exec_fn!(|_n| async {
//         tokio::time::sleep(Duration::from_millis(1000)).await;
//         69
//       }),
//     );
//     graph.create_node(&schema);
//
//     assert_eq!(graph.execute_node(0).await.unwrap(), 69);
//   }
// }
