use std::sync::{Arc, Mutex};

use crate::{io::*, types::*};

pub struct Node {
    pub id: i32,
    pub schema: NodeSchemaRef,
    pub inputs: Mutex<Vec<Input>>,
    pub outputs: Mutex<Vec<Output>>,
}

impl Node {
    pub(crate) fn new(id: i32, schema: &NodeSchemaRef) -> NodeRef {
        let schema = schema.clone();

        let node = Arc::new(Self {
            id,
            schema: schema.clone(),
            inputs: Mutex::new(vec![]),
            outputs: Mutex::new(vec![]),
        });

        schema.build(node.clone());

        node
    }

    pub fn find_input(&self, name: &str) -> Option<Input> {
        self.inputs
            .lock()
            .unwrap()
            .iter()
            .find(|i| i.get_name() == name)
            .map(|i| i.clone())
    }

    pub fn find_output(&self, name: &str) -> Option<Output> {
        self.outputs
            .lock()
            .unwrap()
            .iter()
            .find(|o| o.get_name() == name)
            .map(|o| o.clone())
    }

    pub fn find_data_input(&self, name: &str) -> Option<Arc<DataInput>> {
        self.find_input(name).and_then(|i| {
            if let Input::Data(i) = i {
                Some(i)
            } else {
                None
            }
        })
    }

    pub fn find_data_output(&self, name: &str) -> Option<Arc<DataOutput>> {
        self.find_output(name).and_then(|o| {
            if let Output::Data(o) = o {
                Some(o)
            } else {
                None
            }
        })
    }

    pub fn find_exec_input(&self, name: &str) -> Option<Arc<ExecInput>> {
        self.find_input(name).and_then(|i| {
            if let Input::Exec(i) = i {
                Some(i)
            } else {
                None
            }
        })
    }

    pub fn find_exec_output(&self, name: &str) -> Option<Arc<ExecOutput>> {
        self.find_output(name).and_then(|o| {
            if let Output::Exec(o) = o {
                Some(o)
            } else {
                None
            }
        })
    }

    pub fn add_data_input(&self, input: DataInput) {
        self.inputs
            .lock()
            .unwrap()
            .push(Input::Data(Arc::new(input)));
    }

    pub fn add_exec_input(&self, input: ExecInput) {
        self.inputs
            .lock()
            .unwrap()
            .push(Input::Exec(Arc::new(input)));
    }

    pub fn add_data_output(&self, output: DataOutput) {
        self.outputs
            .lock()
            .unwrap()
            .push(Output::Data(Arc::new(output)));
    }

    pub fn add_exec_output(&self, output: ExecOutput) {
        self.outputs
            .lock()
            .unwrap()
            .push(Output::Exec(Arc::new(output)));
    }
}

#[cfg(test)]
mod test {
    // use super::Node;
    //
    // use crate::{
    //     core::schema::{ExecuteFn, NodeSchema},
    //     exec_fn,
    // };

    // #[tokio::test]
    // async fn can_execute_node_schema() {
    //     let schema = NodeSchema::new_exec("test", |_n| {}, exec_fn!(|_n, _c| async { None }));
    //     let node = Node::new(0, &schema);
    //
    //     if let NodeSchema::Exec(schema) = &*schema {
    //         if let ExecuteFn::Async(func) = schema.execute {
    //             assert_eq!(func(node.clone()).await, 0)
    //         }
    //     }
    // }
}
