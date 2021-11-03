use macrograph::{start_fn, Engine, EngineContext, EngineRef};
use midir::{MidiInput, MidiOutput};

async fn start(_: EngineRef, _: EngineContext) {
    // let mut engine = engine.lock().unwrap();
    // let state: &mut EngineState = engine.state();
}

pub struct EngineState {
    pub input: MidiInput,
    pub output: MidiOutput,
}

pub fn setup_engine() -> Engine {
    Engine::new(
        EngineState {
            input: MidiInput::new("Macrograph Input").unwrap(),
            output: MidiOutput::new("Macrograph Output").unwrap(),
        },
        start_fn!(start),
    )
}
