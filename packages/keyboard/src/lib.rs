pub mod engine;
pub mod key;
pub mod key_event;
mod types;

use self::engine::setup_engine;
use key_event::KeyEvent;
use macrograph_core_types::{fire_fn, Package};

const PRESSED: &str = "Pressed";
const RELEASED: &str = "Released";
const SHIFT: &str = "Shift Pressed";
const CTRL: &str = "Ctrl Pressed";
const ALT: &str = "Alt Pressed";
const META: &str = "Meta Pressed";

#[no_mangle]
pub fn create_package(package: &mut Package) {
    package.set_name("Keyboard");
    package.set_engine(setup_engine());

    for c in 'A'..'Z' {
        package.add_event_schema(
            &c.to_string(),
            |node| {
                node.add_exec_output(PRESSED);
                node.add_exec_output(RELEASED);

                node.add_data_output(SHIFT, false.into());
                node.add_data_output(CTRL, false.into());
                node.add_data_output(ALT, false.into());
                node.add_data_output(META, false.into());
            },
            fire_fn!(|node, event: KeyEvent| {
                node.set_output(SHIFT, event.shift_pressed.into());
                node.set_output(CTRL, event.ctrl_pressed.into());
                node.set_output(ALT, event.alt_pressed.into());
                node.set_output(META, event.meta_pressed.into());

                Some(if event.pressed { PRESSED } else { RELEASED })
            }),
        );
    }
}
