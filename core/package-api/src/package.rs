use crate::{
    engine::EngineConfig,
    types::{BuildFn, FireFn},
    ExecuteFn, NodeSchema,
};

pub struct Package {
    pub name: String,
    pub schemas: Vec<NodeSchema>,
    pub engine_config: Option<EngineConfig>,
    pub runtime: tokio::runtime::Runtime,
}

impl Package {
    pub fn new(name: &str) -> Self {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()
            .unwrap();

        Self {
            name: name.into(),
            schemas: vec![],
            engine_config: None,
            // Multithread with 1 worker thread seems to work whereas current thread doesn't
            runtime,
        }
    }

    pub fn set_engine(&mut self, engine: EngineConfig) {
        self.engine_config = Some(engine);
    }

    fn add_schema(&mut self, mut schema: NodeSchema) {
        schema.package = self.name.to_string();
        self.schemas.push(schema);
    }

    pub fn add_event_schema(&mut self, event: &str, build: BuildFn, fire: FireFn) {
        self.add_schema(NodeSchema::new_event(event, build, fire));
    }

    pub fn add_exec_schema(&mut self, name: &str, build: BuildFn, execute: ExecuteFn<()>) {
        self.add_schema(NodeSchema::new_exec(name, build, execute));
    }

    pub fn add_base_schema(&mut self, name: &str, build: BuildFn, execute: ExecuteFn) {
        self.add_schema(NodeSchema::new_base(name, build, execute));
    }
}
