use crate::core::engine::{Engine, EngineContext};
use crate::core::types::EngineRef;
use crate::start_fn;

use serde_json::Map;
use midir::{MidiInput, MidiOutput};

async fn start(engine: EngineRef, ctx: EngineContext) {
    let mut engine = engine.lock().unwrap();
    let state: &mut EngineState = engine.state();
}

pub struct EngineState {
    pub input: MidiInput,
    pub output: MidiOutput
}

pub fn setup_engine() -> Engine {
    Engine::new(
        EngineState {
            input: MidiInput::new("Macrograph Input").unwrap(),
            output: MidiOutput::new("Macrograph Output").unwrap()
        },
        start_fn!(start)
    )
}
