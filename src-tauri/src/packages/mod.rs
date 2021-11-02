use crate::core::Core;

pub mod keyboard;
mod utils;
mod midi;

pub fn load_packages(core: &mut Core) {
    core.register_package(keyboard::create_package());
    core.register_package(utils::create_package());
    core.register_package(midi::create_package());
}