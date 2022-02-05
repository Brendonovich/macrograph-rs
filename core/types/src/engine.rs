use std::{
    any::Any,
    ops::{Deref, DerefMut},
    sync::Arc,
};
use tokio::sync::{mpsc::UnboundedSender, Mutex, MutexGuard};

use crate::EngineRef;

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
    pub event_channel: UnboundedSender<Event>,
    package: String,
    pub handle: tokio::runtime::Handle,
}

impl EngineContext {
    pub fn send(&self, event: &str, data: impl Any + Send + Sync) {
        self.event_channel
            .send(Event::new(&self.package, event, data));
    }

    pub fn new(
        package: String,
        channel: UnboundedSender<Event>,
        handle: tokio::runtime::Handle,
    ) -> Self {
        Self {
            package,
            event_channel: channel,
            handle,
        }
    }
}

// it's possible that this could make IO and timer operations unusable
// since we are not blocking on the runtime, but instead the handler.
#[macro_export]
macro_rules! run_fn {
    ($name:ident) => {
        |e, ctx| {
            let handle = ctx.handle.clone();
            handle.block_on($name(e, ctx))
        };
    };
}

pub struct EngineState();
pub struct EngineStateGuard<'a, T> {
    guard: MutexGuard<'a, Box<dyn Any + Send>>,
    _phantom: std::marker::PhantomData<T>,
}

impl<'a, T: 'static> Deref for EngineStateGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.guard.downcast_ref().unwrap()
    }
}

impl<'a, T: 'static> DerefMut for EngineStateGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.guard.downcast_mut().unwrap()
    }
}

type RunFn = fn(engine: EngineRef, ctx: EngineContext);

pub struct Engine {
    state: Arc<Mutex<Box<dyn Any + Send>>>,
    pub run: RunFn,
}

impl Engine {
    pub fn new<S: 'static + Send>(state: S, run: RunFn) -> Self {
        Self {
            state: Arc::new(Mutex::new(Box::new(state))),
            run,
        }
    }
    pub async fn state<T: 'static + Send>(&self) -> EngineStateGuard<'_, T> {
        let guard = self.state.lock().await;
        EngineStateGuard {
            guard,
            _phantom: std::marker::PhantomData,
        }
    }
}

// pub trait EngineTrait {
//     pub async fn run(&self, ctx: EngineContext);
// }
