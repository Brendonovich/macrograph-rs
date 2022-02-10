use std::pin::Pin;

use futures::{Stream, StreamExt};
use macrograph_package_api::{engine::EngineContext, EngineRequest};
use obws::{client::ConnectConfig, events::Event, requests::EventSubscription, Client};
use tokio::{select, time};

use crate::{constants::*, types};

pub enum Request {
    SetCurrentScene {
        scene_name: String,
        preview: bool,
    },
    CreateScene {
        scene_name: String,
    },
    RemoveScene {
        scene_name: String,
    },
    SetSceneName {
        scene_name: String,
        new_name: String,
    },
    GetSceneItemID {
        scene_name: String,
        source_name: String,
    },
    CreateSceneItem {
        scene_name: String,
        source_name: String,
        enabled: bool,
    },
    RemoveSceneItem {
        scene_name: String,
        item_id: i64,
    },
    GetSceneItemEnabled {
        scene_name: String,
        item_id: i64,
    },
    SetSceneItemEnabled {
        scene_name: String,
        item_id: i64,
        enabled: bool,
    },
    GetSceneItemLocked {
        scene_name: String,
        item_id: i64,
    },
    SetSceneItemLocked {
        scene_name: String,
        item_id: i64,
        locked: bool,
    },
    GetSceneItemIndex {
        scene_name: String,
        item_id: i64,
    },
    SetSceneItemIndex {
        scene_name: String,
        item_id: i64,
        index: u32,
    },
    ToggleStream,
    StartStream,
    StopStream,
}

enum EngineState {
    Disconnected,
    Connecting,
    Connected {
        client: Client,
        events_stream: Pin<Box<dyn Stream<Item = Event>>>,
    },
}

pub async fn run(mut ctx: EngineContext) {
    let mut state = EngineState::Connecting;

    loop {
        let new_state = match &mut state {
            EngineState::Disconnected => {
                tokio::time::sleep(time::Duration::from_secs(1)).await;
                EngineState::Connecting
            }
            EngineState::Connecting => Client::connect_with_config(ConnectConfig {
                host: "localhost",
                port: 4444,
                password: None as Option<String>,
                event_subscriptions: Some(EventSubscription::ALL),
                broadcast_capacity: None,
            })
            .await
            .map(|client| {
                let events_stream = Box::pin(client.events().unwrap());

                EngineState::Connected {
                    client,
                    events_stream,
                }
            })
            .ok()
            .unwrap_or(EngineState::Disconnected),
            EngineState::Connected {
                client,
                events_stream,
            } => loop {
                select! {
                    Some(ws_event) = events_stream.next() => {
                        use Event::*;

                        match ws_event {
                            SceneCreated { scene_name, is_group } => {
                                ctx.send(SCENE_CREATED, types::SceneCreated{scene_name, is_group});
                            }
                            SceneRemoved { scene_name, is_group } => {
                                ctx.send(SCENE_REMOVED, types::SceneRemoved {scene_name, is_group});
                            }
                            SceneNameChanged { old_scene_name, scene_name} => {
                                ctx.send(SCENE_NAME_CHANGED, types::SceneNameChanged{old_scene_name, scene_name});
                            }
                            CurrentProgramSceneChanged { scene_name } => {
                                ctx.send(PROGRAM_SCENE_CHANGED, scene_name);
                            }
                            CurrentPreviewSceneChanged { scene_name } => {
                                ctx.send(PREVIEW_SCENE_CHANGED, scene_name);
                            }
                            SceneItemCreated {scene_name, source_name, scene_item_id, scene_item_index} => {
                                ctx.send(SCENE_ITEM_CREATED, types::SceneItemCreated{scene_name, source_name, item_id: scene_item_id, item_index: scene_item_index});
                            }
                            SceneItemRemoved {scene_name, input_name, scene_item_id} => {
                                ctx.send(SCENE_ITEM_REMOVED, types::SceneItemRemoved{scene_name, input_name, item_id: scene_item_id});
                            }
                            SceneItemEnableStateChanged { scene_name, scene_item_id, scene_item_enabled } => {
                                ctx.send(SCENE_ITEM_ENABLED_CHANGED, types::SceneItemEnableChanged{scene_name, item_id: scene_item_id, enabled: scene_item_enabled});
                            }
                            SceneItemLockStateChanged { scene_name, scene_item_id, scene_item_locked } => {
                                ctx.send(SCENE_ITEM_LOCK_CHANGED, types::SceneItemLockChanged{scene_name, item_id: scene_item_id, locked: scene_item_locked});
                            }
                            SceneItemSelected { scene_name, scene_item_id } => {
                                ctx.send(SCENE_ITEM_SELECTED, types::SceneItemSelected{scene_name, item_id: scene_item_id});
                            }
                            ServerStopping => {
                                ctx.send(OBS_SERVER_STOPPING, ());
                            }
                            ServerStopped => {
                                ctx.send(OBS_SERVER_STOPPED, ());
                                break EngineState::Disconnected
                            }
                            _ => {}
                        };
                    }
                    Some(request) = ctx.receive_request() => {
                        handle_request(client, request).await;
                    }
                };
            },
        };

        state = new_state;
    }
}

async fn handle_request(client: &mut Client, request: EngineRequest) {
    match request {
        EngineRequest::Send(data) => {
            let _data = match data.downcast::<Request>() {
                Ok(r) => {
                    use Request::*;

                    match *r {
                        SetCurrentScene {
                            scene_name,
                            preview,
                        } => {
                            if preview {
                                client
                                    .scenes()
                                    .set_current_preview_scene(&scene_name)
                                    .await;
                            } else {
                                client
                                    .scenes()
                                    .set_current_program_scene(&scene_name)
                                    .await;
                            }
                        }
                        CreateScene { scene_name } => {
                            client.scenes().create_scene(&scene_name).await;
                        }
                        RemoveScene { scene_name } => {
                            client.scenes().remove_scene(&scene_name).await;
                        }
                        SetSceneName {
                            scene_name,
                            new_name,
                        } => {
                            client
                                .scenes()
                                .set_scene_name(&scene_name, &new_name)
                                .await;
                        }
                        GetSceneItemID {
                            scene_name,
                            source_name,
                        } => {
                            client
                                .scene_items()
                                .get_scene_item_id(&scene_name, &source_name)
                                .await; // TODO
                        }
                        CreateSceneItem {
                            scene_name,
                            source_name,
                            enabled,
                        } => {
                            client
                                .scene_items()
                                .create_scene_item(obws::requests::CreateSceneItem {
                                    scene_name: &scene_name,
                                    source_name: &source_name,
                                    scene_item_enabled: Some(enabled),
                                })
                                .await; // TODO
                        }
                        RemoveSceneItem {
                            scene_name,
                            item_id,
                        } => {
                            client
                                .scene_items()
                                .remove_scene_item(&scene_name, item_id)
                                .await; // TODO
                        }
                        GetSceneItemEnabled {
                            scene_name,
                            item_id,
                        } => {
                            client
                                .scene_items()
                                .get_scene_item_enabled(&scene_name, item_id)
                                .await
                                .unwrap(); // TODO
                        }
                        SetSceneItemEnabled {
                            scene_name,
                            item_id: scene_item_id,
                            enabled,
                        } => {
                            client
                                .scene_items()
                                .set_scene_item_enabled(obws::requests::SetSceneItemEnabled {
                                    scene_name: &scene_name,
                                    scene_item_id,
                                    scene_item_enabled: enabled,
                                })
                                .await; // TODO
                        }
                        GetSceneItemLocked {
                            scene_name,
                            item_id,
                        } => {
                            client
                                .scene_items()
                                .get_scene_item_locked(&scene_name, item_id)
                                .await; // TODO
                        }
                        SetSceneItemLocked {
                            scene_name,
                            item_id: scene_item_id,
                            locked,
                        } => {
                            client
                                .scene_items()
                                .set_scene_item_locked(obws::requests::SetSceneItemLocked {
                                    scene_name: &scene_name,
                                    scene_item_id,
                                    scene_item_locked: locked,
                                })
                                .await; // TODO
                        }
                        GetSceneItemIndex {
                            scene_name,
                            item_id,
                        } => {
                            client
                                .scene_items()
                                .get_scene_item_index(&scene_name, item_id)
                                .await; // TODO
                        }
                        SetSceneItemIndex {
                            scene_name,
                            item_id: scene_item_id,
                            index,
                        } => {
                            client
                                .scene_items()
                                .set_scene_item_index(obws::requests::SetSceneItemIndex {
                                    scene_name: &scene_name,
                                    scene_item_id,
                                    scene_item_index: index,
                                })
                                .await; // TODO
                        }
                        ToggleStream => {
                            client.streaming().toggle_stream().await; // TODO
                        }
                        StartStream => {
                            client.streaming().start_stream().await;
                        }
                        StopStream => {
                            client.streaming().stop_stream().await;
                        }
                        _ => {}
                    };
                }
                Err(data) => {}
            };
        }
        _ => {}
    }
}
