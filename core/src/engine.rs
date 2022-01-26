use futures::future::BoxFuture;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::any::Any;
use tokio::sync::mpsc::UnboundedSender;

use crate::EngineRef;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Event {
    pub package: String,
    pub event: String,
    pub data: Value,
}

impl Event {
    pub fn new(package: &str, event: &str, data: serde_json::Value) -> Self {
        Self {
            package: package.to_string(),
            event: event.to_string(),
            data,
        }
    }
}

pub struct EngineContext {
    pub event_channel: UnboundedSender<Event>,
    package: String,
}

impl EngineContext {
    pub fn send(&self, event: &str, data: Value) {
        self.event_channel.send(Event {
            package: self.package.clone(),
            event: event.to_string(),
            data,
        });
    }

    pub fn new(package: &str, channel: &UnboundedSender<Event>) -> Self {
        Self {
            package: package.to_string(),
            event_channel: channel.clone(),
        }
    }
}

#[macro_export]
macro_rules! run_fn {
    ($name:ident) => {
        |e, ctx| Box::pin($name(e, ctx))
    };
}

type RunFn = fn(engine: EngineRef, ctx: EngineContext) -> BoxFuture<'static, ()>;

pub struct Engine {
    state: Box<dyn Any + Send>,
    pub run: RunFn,
}

impl Engine {
    pub fn new<S: 'static + Send>(state: S, run: RunFn) -> Self {
        Self {
            state: Box::new(state),
            run,
        }
    }

    pub fn state<S: 'static + Send>(&mut self) -> &mut S {
        self.state.downcast_mut().unwrap()
    }
}
