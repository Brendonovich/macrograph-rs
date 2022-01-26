use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{Engine, EngineRef, NodeSchema, NodeSchemaRef};

pub struct Package {
    pub name: String,
    pub(crate) schemas: Vec<NodeSchemaRef>,
    pub engine: Option<EngineRef>,
}

impl Package {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            schemas: vec![],
            engine: None,
        }
    }

    pub fn add_schema(&mut self, mut schema: NodeSchema) {
        schema.package = self.name.to_string();
        self.schemas.push(Arc::new(schema));
    }

    pub fn set_engine(&mut self, engine: Engine) {
        self.engine = Some(Arc::new(Mutex::new(engine)));
    }

    pub fn schema(&self, name: &str) -> Option<&NodeSchemaRef> {
        self.schemas.iter().find(|s| s.id == name)
    }

    pub fn schema_mut(&mut self, name: &str) -> Option<&mut NodeSchemaRef> {
        self.schemas.iter_mut().find(|s| s.id == name)
    }
}
