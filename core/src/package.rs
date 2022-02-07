use std::sync::Arc;

use macrograph_package_api::{
    engine::{InitialEngineState, RunFn},
    package::Package as ApiPackage,
    EngineRequest,
};
use tokio::sync::mpsc::UnboundedSender;

use crate::schema::NodeSchema;

pub enum Engine {
    Created {
        run: RunFn,
        state: InitialEngineState,
    },
    Running {
        request_sender: UnboundedSender<EngineRequest>,
    },
}

impl Engine {
    pub fn new(run: RunFn, state: InitialEngineState) -> Self {
        Self::Created { run, state }
    }
}

pub struct Package {
    pub name: String,
    pub schemas: Vec<Arc<NodeSchema>>,
    pub engine: Option<Engine>,
    pub runtime: tokio::runtime::Runtime,
}

impl Package {
    pub fn add_schema(&mut self, schema: NodeSchema) {
        self.schemas.push(Arc::new(schema));
    }

    pub fn schema(&self, name: &str) -> Option<&Arc<NodeSchema>> {
        self.schemas.iter().find(|s| s.name == name)
    }
}

impl From<ApiPackage> for Package {
    fn from(api_package: ApiPackage) -> Self {
        Self {
            name: api_package.name,
            schemas: api_package
                .schemas
                .into_iter()
                .map(|s| Arc::new(s.into()))
                .collect(),
            engine: api_package
                .engine_config
                .map(|config| Engine::new(config.run, config.state)),
            runtime: api_package.runtime,
        }
    }
}
