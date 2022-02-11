use std::{any::Any, ops::Deref};

use tokio::{
    runtime::{EnterGuard, Handle},
    sync::{
        mpsc::UnboundedSender,
        oneshot::{self, Sender},
    },
};

use crate::types::{AsyncExecuteFn, BuildFn, FireFn, SyncExecuteFn};

pub enum ExecuteFn<T = Option<&'static str>> {
    Sync(SyncExecuteFn<T>),
    Async(AsyncExecuteFn<T>),
}

pub enum NodeSchemaType {
    Base { execute: ExecuteFn },
    Exec { execute: ExecuteFn<()> },
    Pure { execute: ExecuteFn<()> },
    Event { fire: FireFn },
}

pub struct NodeSchema {
    pub name: String,
    pub package: String,
    pub build: BuildFn,
    pub inner: NodeSchemaType,
}

impl Deref for NodeSchema {
    type Target = NodeSchemaType;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl NodeSchema {
    pub fn new_exec(name: &str, build: BuildFn, execute: ExecuteFn<()>) -> Self {
        Self {
            name: name.into(),
            build,
            package: String::new(),
            inner: NodeSchemaType::Exec { execute },
        }
    }

    pub fn new_base(name: &str, build: BuildFn, execute: ExecuteFn) -> Self {
        Self {
            name: name.into(),
            build,
            package: String::new(),
            inner: NodeSchemaType::Base { execute },
        }
    }

    pub fn new_event(name: &str, build: BuildFn, fire: FireFn) -> Self {
        Self {
            name: name.into(),
            build,
            package: String::new(),
            inner: NodeSchemaType::Event { fire },
        }
    }
}

pub type EngineRequestData = Box<dyn Any + Send>;

#[derive(Debug)]
pub enum EngineRequest {
    Send(EngineRequestData),
    Invoke(EngineRequestData, Sender<EngineRequestData>),
}

pub struct ExecuteContext {
    sender: Option<UnboundedSender<EngineRequest>>,
    handle: Handle,
}

impl ExecuteContext {
    pub fn new(sender: Option<UnboundedSender<EngineRequest>>, handle: Handle) -> Self {
        Self { sender, handle }
    }

    pub fn enter_handle(&self) -> EnterGuard<'_> {
        self.handle.enter()
    }

    pub fn send<T: Any + Send>(&self, data: T) {
        if let Some(sender) = &self.sender {
            sender.send(EngineRequest::Send(Box::new(data))).unwrap();
        }
    }

    pub async fn invoke<T: Any>(&self, data: impl Any + Send) -> Option<T> {
        if let Some(sender) = &self.sender {
            let (tx, rx) = oneshot::channel();
            sender
                .send(EngineRequest::Invoke(Box::new(data), tx))
                .unwrap();
            let ret = rx.await.ok();
            ret.and_then(|data| data.downcast::<Option<T>>().ok()).and_then(|v| *v)
        } else {
            None
        }
    }
}

#[macro_export]
macro_rules! exec_fn {
  (|$t:ident, $ctx:ident| async $($body:tt)*) => {{
    $crate::schema::ExecuteFn::Async(|$t, $ctx|
        Box::pin(async move {
            let _guard = $ctx.enter_handle();
            async $($body)*.await
       }))
  }};
  (|$t:ident, $ctx:ident| $($body:tt)*) => {{
    $crate::schema::ExecuteFn::Sync(|$t, $ctx| $($body)*)
  }};
}

#[macro_export]
macro_rules! fire_fn {
  (|$node: ident, $event:ident: &$event_type:ident| $($body:tt)*) => {{
    |$node, $event| {
      let $event = $event.downcast_ref::<$event_type>().unwrap();
      $($body)*
    }
  }};
}
