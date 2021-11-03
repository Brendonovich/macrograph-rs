use macrograph::Package;

pub fn create_packages() -> Vec<Package> {
    vec![
        macrograph_pkg_keyboard::create_package(),
        macrograph_pkg_midi::create_package(),
        macrograph_pkg_utils::create_package(),
    ]
}
