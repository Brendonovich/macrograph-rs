use macrograph_core::{exec_fn, DataOutput, NodeSchema, Package};

use engine::{setup_engine, EngineState};

mod engine;

pub fn create_package() -> Package {
    let mut package = Package::new("midi");

    package.set_engine(setup_engine());

    package
}
