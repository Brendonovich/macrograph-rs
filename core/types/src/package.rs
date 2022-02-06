use std::sync::Arc;

use crate::{schema::NodeSchema, BaseExecuteFn, BuildFn, Engine, EngineRef, ExecuteFn, FireFn};

pub struct Package {
    pub name: String,
    pub schemas: Vec<Arc<NodeSchema>>,
    pub engine: Option<EngineRef>,
    pub runtime: tokio::runtime::Runtime,
}

impl Package {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            schemas: vec![],
            engine: None,
            // Multithread with 1 worker thread seems to work whereas current thread doesn't
            runtime: tokio::runtime::Builder::new_multi_thread()
                .worker_threads(1)
                .enable_all()
                .build()
                .unwrap(),
        }
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.into();
    }

    pub fn add_schema(&mut self, mut schema: NodeSchema) {
        schema.package = self.name.to_string();
        self.schemas.push(Arc::new(schema));
    }

    pub fn add_event_schema(&mut self, event: &str, build: BuildFn, fire: FireFn) {
        self.add_schema(NodeSchema::new_event(event, build, fire));
    }

    pub fn add_exec_schema(&mut self, name: &str, build: BuildFn, execute: ExecuteFn) {
        self.add_schema(NodeSchema::new_exec(name, build, execute));
    }

    pub fn add_base_schema(&mut self, name: &str, build: BuildFn, execute: BaseExecuteFn) {
        self.add_schema(NodeSchema::new_base(name, build, execute));
    }

    pub fn set_engine(&mut self, engine: Engine) {
        self.engine = Some(Arc::new(engine));
    }

    pub fn schema(&self, name: &str) -> Option<&Arc<NodeSchema>> {
        self.schemas.iter().find(|s| s.name == name)
    }
}
