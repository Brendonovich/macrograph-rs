use std::sync::{Arc, Mutex};

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
        match schema {
            NodeSchema::Event(ref mut schema) => schema.package = self.name.to_string(),
            NodeSchema::Exec(ref mut schema) => schema.package = self.name.to_string(),
        };

        self.schemas.push(Arc::new(schema));
    }

    pub fn set_engine(&mut self, engine: Engine) {
        self.engine = Some(Arc::new(Mutex::new(engine)));
    }

    pub fn schema(&self, name: &str) -> Option<&NodeSchemaRef> {
        self.schemas.iter().find(|s| s.get_id() == name)
    }
}
