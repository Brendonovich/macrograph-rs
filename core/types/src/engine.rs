use futures::future::BoxFuture;
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
    pub handle: tokio::runtime::Handle,
    package: String,
}

impl EngineContext {
    pub fn send(&self, event: &str, data: impl Any + Send + Sync) {
        self.event_channel
            .send(Event::new(&self.package, event, data));
    }

    pub fn new(
        package: &str,
        channel: &UnboundedSender<Event>,
        handle: tokio::runtime::Handle,
    ) -> Self {
        Self {
            package: package.to_string(),
            event_channel: channel.clone(),
            handle,
        }
    }
}

#[macro_export]
macro_rules! run_fn {
    ($name:ident) => {
        |e, ctx| Box::pin($name(e, ctx));
    };
}

type RunFn = fn(engine: EngineRef, ctx: EngineContext) -> BoxFuture<'static, ()>;

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

pub struct Engine {
    state: Arc<Mutex<Box<dyn Any + Send>>>,
    pub runtime: tokio::runtime::Runtime,
    pub run: RunFn,
}

impl Engine {
    pub fn new<S: 'static + Send>(state: S, run: RunFn) -> Self {
        Self {
            state: Arc::new(Mutex::new(Box::new(state))),
            runtime: tokio::runtime::Runtime::new().unwrap(),
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
