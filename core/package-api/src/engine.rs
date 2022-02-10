use std::any::Any;

use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::EngineRequest;

pub type InitialEngineState = Option<Box<dyn Any + Send + Sync>>;

#[derive(Debug)]
pub struct Event {
    pub package: String,
    pub event: String,
    pub data: Box<dyn Any + Send + Sync>,
}

impl Event {
    pub fn new(package: &str, event: &str, data: impl Any + Send + Sync) -> Self {
        Self {
            package: package.to_string(),
            event: event.to_string(),
            data: Box::new(data),
        }
    }
}

pub struct EngineContext {
    pub initial_state: InitialEngineState,
    // Channel to send events to the Core for triggering Event nodes
    pub event_sender: UnboundedSender<Event>,
    // Channel to receive requests from nodes
    pub request_receiver: UnboundedReceiver<EngineRequest>,
    pub package: String,
    pub handle: tokio::runtime::Handle,
}

impl EngineContext {
    pub fn initial_state<T: 'static>(&mut self) -> Box<T> {
        self.initial_state
            .take()
            .map(|s| s.downcast::<T>().unwrap())
            .unwrap()
    }

    pub fn send(&self, event: &str, data: impl Any + Send + Sync) {
        self.event_sender
            .send(Event::new(&self.package, event, data));
    }

    pub async fn receive_request(&mut self) -> Option<EngineRequest> {
        self.request_receiver.recv().await
    }
}

pub struct EngineConfig {
    pub run: RunFn,
    pub state: InitialEngineState,
}

// it's possible that this could make IO and timer operations unusable
// since we are not blocking on the runtime, but instead the handler.
#[macro_export]
macro_rules! run_fn {
    ($name:ident) => {
        |mut ctx| {
            let handle = ctx.handle.clone();
            handle.block_on($name(ctx))
        };
    };
}

pub type RunFn = fn(ctx: EngineContext);
