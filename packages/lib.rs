pub fn register_packages(core: &mut macrograph_core::Core) {
    core.register_package(mg_pkg_keyboard::create_package());
    core.register_package(mg_pkg_logic::create_package());
    core.register_package(mg_pkg_midi::create_package());
    core.register_package(mg_pkg_utils::create_package());
    core.register_package(mg_pkg_obs::create_package());
}