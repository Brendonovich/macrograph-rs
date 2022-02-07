use futures::{pin_mut, StreamExt};
use macrograph_package_api::engine::EngineContext;
use obws::{client::ConnectConfig, events::Event, requests::EventSubscription, Client};
use tokio::select;

use crate::constants::{PREVIEW_SCENE_CHANGED, PROGRAM_SCENE_CHANGED};

pub async fn run(mut ctx: EngineContext) {
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

    pin_mut!(stream);

    loop {
        select! {
            Some(ws_event) = stream.next() => {
                 match ws_event {
                    Event::CurrentProgramSceneChanged { scene_name } => {
                        ctx.send(PROGRAM_SCENE_CHANGED, scene_name);
                    }
                    Event::CurrentPreviewSceneChanged { scene_name } => {
                        ctx.send(PREVIEW_SCENE_CHANGED, scene_name);
                    }
                    _ => {}
                };
            }
            Some(request) = ctx.receive_request() => {
                println!("{:?}", request);
            }
        }
    }
}
