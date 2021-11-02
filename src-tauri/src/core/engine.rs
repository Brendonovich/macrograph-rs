use futures::future::BoxFuture;
use serde_json::{Map, Value};
use tokio::sync::mpsc::UnboundedSender;
use serde::{Serialize, Deserialize};

use super::types::EngineRef;
use crate::core::{Request, RequestData};
use std::any::Any;
use std::sync::Mutex;

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
  pub event_channel: UnboundedSender<Request>,
  package: String
}

impl EngineContext {
  pub fn send(&self, event: &str, data: Value) {
    self.event_channel.send(Request {
      id: -1,
      data: RequestData::Event(Event {
        package: self.package.clone(),
        event: event.to_string(),
        data
      })
    });
  }

  pub fn new(package: &str, channel: &UnboundedSender<Request>) -> Self{
    Self {
      package: package.to_string(),
      event_channel: channel.clone()
    }
  }
}

#[macro_export]
macro_rules! start_fn {
  ($name:ident) => {
    |e, ctx| Box::pin($name(e, ctx))
  };
}

type StartFn = fn(engine: EngineRef, ctx: EngineContext) -> BoxFuture<'static, ()>;

pub struct Engine {
  state: Box<dyn Any + Send>,
  pub start: StartFn,
}

impl Engine {
  pub fn new<S: 'static + Send>(state: S, start: StartFn) -> Self {
    Self {
      state: Box::new(state),
      start
    }
  }

  pub fn state<S: 'static + Send>(&mut self) -> &mut S {
    self.state.downcast_mut().unwrap()
  }
}
