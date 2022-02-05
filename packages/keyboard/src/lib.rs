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
pub fn create_package() -> Package {
    let mut package = Package::new("Keyboard");
    package.set_engine(setup_engine());

    for c in 'A'..'Z' {
        package.add_event_schema(
            &c.to_string(),
            |s| {
                s.exec_output(PRESSED);
                s.exec_output(RELEASED);
                
                s.data_output(SHIFT, false.into());
                s.data_output(CTRL, false.into());
                s.data_output(ALT, false.into());
                s.data_output(META, false.into());
            },
            fire_fn!(|io, e: KeyEvent| {
                io.set_bool(SHIFT, e.shift_pressed);
                io.set_bool(CTRL, e.ctrl_pressed);
                io.set_bool(ALT, e.alt_pressed);
                io.set_bool(META, e.meta_pressed);

                Some(if e.pressed { PRESSED } else { RELEASED })
            }),
        );
    }

    package
}
