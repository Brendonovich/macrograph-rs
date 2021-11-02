use crate::{
    core::{
        engine::{Engine, EngineContext, Event},
        types::EngineRef,
    },
    start_fn,
};
use rdev::{listen, EventType, Key as RDevKey};
use serde_json::Map;

use super::{key::Key, key_event::KeyEvent};

const SHIFT_PRESSED: &str = "";
const CTRL_PRESSED: &str = "";
const ALT_PRESSED: &str = "";
const META_PRESSED: &str = "";

async fn start(engine: EngineRef, ctx: EngineContext) {
    listen(move |event| {
        let mut engine = engine.lock().unwrap();
        let engine_state: &mut State = engine.state();

        match event.event_type {
            EventType::KeyRelease(key) => {
                match key {
                    RDevKey::ShiftLeft | RDevKey::ShiftRight => engine_state.shift_pressed = false,
                    RDevKey::MetaLeft | RDevKey::MetaRight => engine_state.meta_pressed = false,
                    RDevKey::ControlLeft | RDevKey::ControlRight => engine_state.ctrl_pressed = false,
                    RDevKey::Alt => engine_state.alt_pressed = false,
                    _ => {}
                };

                if let Some(key) = Key::from_rdev(key) {
                    let event_name = format!("key_{}", key.to_string().to_lowercase());

                    let key_event = KeyEvent {
                        key,
                        shift_pressed: engine_state.shift_pressed,
                        ctrl_pressed: engine_state.ctrl_pressed,
                        alt_pressed: engine_state.alt_pressed,
                        meta_pressed: engine_state.meta_pressed,
                    };

                    ctx.send(&event_name, key_event.to_value());
                }
            }
            EventType::KeyPress(key) => {
                match key {
                    RDevKey::ShiftLeft | RDevKey::ShiftRight => engine_state.shift_pressed = true,
                    RDevKey::MetaLeft | RDevKey::MetaRight => engine_state.meta_pressed = true,
                    RDevKey::ControlLeft | RDevKey::ControlRight => engine_state.ctrl_pressed = true,
                    RDevKey::Alt => engine_state.alt_pressed = true,
                    _ => {}
                };

                if let Some(key) = Key::from_rdev(key) {
                    let event_name = format!("key_{}", key.to_string().to_lowercase());

                    let key_event = KeyEvent {
                        key,
                        shift_pressed: engine_state.shift_pressed,
                        ctrl_pressed: engine_state.ctrl_pressed,
                        alt_pressed: engine_state.alt_pressed,
                        meta_pressed: engine_state.meta_pressed,
                    };

                    ctx.send(&event_name, key_event.to_value());
                }
            }
            _ => {}
        };
    }).unwrap();
}

pub struct State {
    shift_pressed: bool,
    ctrl_pressed: bool,
    alt_pressed: bool,
    meta_pressed: bool,
}

pub fn setup_engine() -> Engine {
    Engine::new(
        State {
            shift_pressed: false,
            ctrl_pressed: false,
            alt_pressed: false,
            meta_pressed: false,
        },
        start_fn!(start),
    )
}
