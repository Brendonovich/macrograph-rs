use macrograph_core_types::Package;

use engine::setup_engine;

mod engine;

#[no_mangle]
pub fn create_package() -> Package {
    let mut package = Package::new("MIDI");
    
    package.set_engine(setup_engine());
    
    package
}
