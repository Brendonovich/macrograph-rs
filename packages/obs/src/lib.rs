pub mod constants;
pub mod engine;
pub mod schemas;
mod types;

use engine::run;
use macrograph_package_api::{engine::EngineConfig, package::Package, run_fn};
use schemas::create_schemas;

#[no_mangle]
pub fn create_package() -> Package {
    let mut package = Package::new("OBS");

    package.set_engine(EngineConfig {
        run: run_fn!(run),
        state: None,
    });

    create_schemas(&mut package);

    package
}
