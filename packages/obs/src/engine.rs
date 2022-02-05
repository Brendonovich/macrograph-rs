use futures::{pin_mut, StreamExt};
use macrograph_core_types::{run_fn, Engine, EngineContext, EngineRef};
use obws::{client::ConnectConfig, events::Event, requests::EventSubscription, Client};

use crate::constants::{PREVIEW_SCENE_CHANGED, PROGRAM_SCENE_CHANGED};

async fn main(engine: EngineRef, ctx: EngineContext) {
    // necessary to release the lock on state
    let stream = {
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

        let stream = client.events().unwrap();

        state.client = Some(client);
        stream
    };

    pin_mut!(stream);

    while let Some(event) = stream.next().await {
        println!("{:?}", event);
        match event {
            Event::CurrentProgramSceneChanged { scene_name } => {
                ctx.send(PROGRAM_SCENE_CHANGED, scene_name);
            }
            Event::CurrentPreviewSceneChanged { scene_name } => {
                ctx.send(PREVIEW_SCENE_CHANGED, scene_name);
            }
            _ => {}
        };
    }
}

pub struct State {
    pub client: Option<Client>,
}

pub fn setup_engine() -> Engine {
    Engine::new(State { client: None }, run_fn!(main))
}
