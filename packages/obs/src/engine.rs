use macrograph_core::{run_fn, Engine, EngineContext, EngineRef};
use obws::Client;

async fn run(engine: EngineRef, _ctx: EngineContext) {
    let client = Client::connect("localhost", 4444, None as Option<String>).await.unwrap();

    let mut engine = engine.lock().await;
    let mut state: &mut State = engine.state();
    state.client = Some(client);
}

pub struct State {
    pub client: Option<Client>,
}

pub fn setup_engine() -> Engine {
    Engine::new(State { client: None }, run_fn!(run))
}
