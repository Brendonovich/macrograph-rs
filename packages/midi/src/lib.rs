use macrograph_core::Package;

use engine::setup_engine;

mod engine;

pub fn create_package() -> Package {
    let mut package = Package::new("midi");

    package.set_engine(setup_engine());

    package
}
