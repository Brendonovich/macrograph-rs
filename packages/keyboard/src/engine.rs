use std::sync::Arc;

use crate::{key::Key, key_event::KeyEvent};
use macrograph_core::{run_fn, Engine, EngineContext, EngineRef};
use rdev::{listen, Event, EventType, Key as RDevKey};
use tokio::sync::{
    mpsc::{self, UnboundedReceiver},
    Mutex,
};

async fn run(engine: EngineRef, ctx: EngineContext) {
    let receiver = {
        let mut mutex = engine.lock().await;
        let state: &mut State = mutex.state();
        state.message_receiver.clone()
    };

    while let Some(event) = receiver.lock().await.recv().await {
        let mut engine = engine.lock().await;
        let engine_state: &mut State = engine.state();

        match event.event_type {
            EventType::KeyRelease(key) => {
                match key {
                    RDevKey::ShiftLeft | RDevKey::ShiftRight => engine_state.shift_pressed = false,
                    RDevKey::MetaLeft | RDevKey::MetaRight => engine_state.meta_pressed = false,
                    RDevKey::ControlLeft | RDevKey::ControlRight => {
                        engine_state.ctrl_pressed = false
                    }
                    RDevKey::Alt => engine_state.alt_pressed = false,
                    _ => {}
                };

                if let Some(key) = Key::from_rdev(key) {
                    let event_name = format!("key_{}", key.to_string().to_lowercase());

                    let key_event = KeyEvent {
                        key,
                        pressed: false,
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
                    RDevKey::ControlLeft | RDevKey::ControlRight => {
                        engine_state.ctrl_pressed = true
                    }
                    RDevKey::Alt => engine_state.alt_pressed = true,
                    _ => {}
                };

                if let Some(key) = Key::from_rdev(key) {
                    let event_name = format!("key_{}", key.to_string().to_lowercase());

                    let key_event = KeyEvent {
                        key,
                        pressed: true,
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
    }
}

pub struct State {
    shift_pressed: bool,
    ctrl_pressed: bool,
    alt_pressed: bool,
    meta_pressed: bool,
    message_receiver: Arc<Mutex<UnboundedReceiver<Event>>>,
}

pub fn setup_engine() -> Engine {
    let (tx, rx) = mpsc::unbounded_channel::<Event>();

    listen(move |event| {
        tx.send(event).unwrap();
    })
    .unwrap();

    Engine::new(
        State {
            shift_pressed: false,
            ctrl_pressed: false,
            alt_pressed: false,
            meta_pressed: false,
            message_receiver: Arc::new(Mutex::new(rx)),
        },
        run_fn!(run),
    )
}
