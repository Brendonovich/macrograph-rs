use macrograph_package_api::schema::NodeSchema as NodeSchemaInner;
use macrograph_package_api::BuildSchema;
use std::{ops::Deref, sync::Weak};
use tokio::sync::Mutex;
use weak_table::PtrWeakHashSet;

use crate::node::Node;

pub struct NodeSchema {
    pub instances: Mutex<PtrWeakHashSet<Weak<Node>>>,
    inner: NodeSchemaInner,
}

impl Deref for NodeSchema {
    type Target = NodeSchemaInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl NodeSchema {
    pub fn build(&self, schema: &mut BuildSchema) {
        use macrograph_package_api::NodeSchemaType::*;

        match ***self {
            Exec { .. } => {
                schema.exec_input("");
                schema.exec_output("");
            }
            _ => {}
        }

        (self.build)(schema);
    }
}

impl From<NodeSchemaInner> for NodeSchema {
    fn from(api_node_schema: NodeSchemaInner) -> Self {
        Self {
            instances: Mutex::new(PtrWeakHashSet::new()),
            inner: NodeSchemaInner::from(api_node_schema),
        }
    }
}
