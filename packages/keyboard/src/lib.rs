pub mod engine;
pub mod key;
pub mod key_event;
mod types;

use engine::run;

use key_event::KeyEvent;
use macrograph_package_api::{engine::EngineConfig, fire_fn, package::Package, run_fn};
use rdev::{listen, Event};
use tokio::sync::mpsc;

const PRESSED: &str = "Pressed";
const RELEASED: &str = "Released";
const SHIFT: &str = "Shift Pressed";
const CTRL: &str = "Ctrl Pressed";
const ALT: &str = "Alt Pressed";
const META: &str = "Meta Pressed";

#[no_mangle]
pub fn create_package() -> Package {
    let mut package = Package::new("Keyboard");

    package.set_engine({
        let (tx, rx) = mpsc::unbounded_channel::<Event>();

        let cb = || {
            listen(move |event| {
                tx.send(event).unwrap();
            })
            .unwrap()
        };

        // macos isn't blocking
        if cfg!(target_os = "macos") {
            cb();
        } else {
            std::thread::spawn(cb);
        }

        EngineConfig {
            run: run_fn!(run),
            state: Some(Box::new(rx)),
        }
    });

    for c in 'A'..'Z' {
        package.add_event_schema(
            &c.to_string(),
            |s| {
                s.exec_output(PRESSED);
                s.exec_output(RELEASED);

                s.bool_output(SHIFT);
                s.bool_output(CTRL);
                s.bool_output(ALT);
                s.bool_output(META);
            },
            fire_fn!(|io, e: &KeyEvent| {
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
