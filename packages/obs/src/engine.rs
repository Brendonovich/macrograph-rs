use futures::{pin_mut, stream::StreamExt};
use macrograph_core_types::{run_fn, Engine, EngineContext, EngineRef};
use obws::{client::ConnectConfig, requests::EventSubscription, Client};

async fn run(engine: EngineRef, ctx: EngineContext) {
    // I hate this but currently it's the only thing that works
    let _guard = ctx.handle.enter();
    let mut state = engine.state::<State>().await;

    let client = Client::connect_with_config(ConnectConfig {
        host: "localhost",
        port: 4444,
        password: None as Option<String>,
        event_subscriptions: Some(EventSubscription::ALL),
        broadcast_capacity: None,
    })
    .await
    .unwrap();
    
    state.client = Some(client);
}

pub struct State {
    pub client: Option<Client>,
}

pub fn setup_engine() -> Engine {
    Engine::new(State { client: None }, run_fn!(run))
}
