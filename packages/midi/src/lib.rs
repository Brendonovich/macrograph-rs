use macrograph_package_api::package::Package;


mod engine;

#[no_mangle]
pub fn create_package() -> Package {
    let mut package = Package::new("MIDI");

    package
}
