use crate::{key::Key, key_event::KeyEvent};
use macrograph_package_api::engine::EngineContext;
use rdev::{Event, EventType, Key as RDevKey};
use tokio::sync::mpsc;

pub type EngineInitialState = mpsc::UnboundedReceiver<Event>;

pub struct EngineState {
    shift_pressed: bool,
    ctrl_pressed: bool,
    alt_pressed: bool,
    meta_pressed: bool,
}

pub async fn run(mut ctx: EngineContext) {
    let mut receiver = ctx
        .initial_state
        .take()
        .map(|s| s.downcast::<EngineInitialState>().unwrap())
        .unwrap();

    let mut state = EngineState {
        shift_pressed: false,
        ctrl_pressed: false,
        alt_pressed: false,
        meta_pressed: false,
    };

    while let Some(event) = receiver.recv().await {
        match event.event_type {
            EventType::KeyRelease(key) => {
                match key {
                    RDevKey::ShiftLeft | RDevKey::ShiftRight => state.shift_pressed = false,
                    RDevKey::MetaLeft | RDevKey::MetaRight => state.meta_pressed = false,
                    RDevKey::ControlLeft | RDevKey::ControlRight => state.ctrl_pressed = false,
                    RDevKey::Alt => state.alt_pressed = false,
                    _ => {}
                };

                if let Some(key) = Key::from_rdev(key) {
                    let event_name = format!("{}", key.to_string().to_uppercase());

                    let key_event = KeyEvent {
                        key,
                        pressed: false,
                        shift_pressed: state.shift_pressed,
                        ctrl_pressed: state.ctrl_pressed,
                        alt_pressed: state.alt_pressed,
                        meta_pressed: state.meta_pressed,
                    };

                    ctx.send(&event_name, key_event);
                }
            }
            EventType::KeyPress(key) => {
                match key {
                    RDevKey::ShiftLeft | RDevKey::ShiftRight => state.shift_pressed = true,
                    RDevKey::MetaLeft | RDevKey::MetaRight => state.meta_pressed = true,
                    RDevKey::ControlLeft | RDevKey::ControlRight => state.ctrl_pressed = true,
                    RDevKey::Alt => state.alt_pressed = true,
                    _ => {}
                };

                if let Some(key) = Key::from_rdev(key) {
                    let event_name = format!("{}", key.to_string().to_uppercase());

                    let key_event = KeyEvent {
                        key,
                        pressed: true,
                        shift_pressed: state.shift_pressed,
                        ctrl_pressed: state.ctrl_pressed,
                        alt_pressed: state.alt_pressed,
                        meta_pressed: state.meta_pressed,
                    };

                    ctx.send(&event_name, key_event);
                }
            }
            _ => {}
        };
    }
}
