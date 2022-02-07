use std::{any::Any, ops::Deref};

use tokio::runtime::EnterGuard;

use crate::types::{AsyncExecuteFn, BuildFn, FireFn, SyncExecuteFn};

pub enum ExecuteFn<T = Option<&'static str>> {
    Sync(SyncExecuteFn<T>),
    Async(AsyncExecuteFn<T>),
}

pub enum NodeSchemaType {
    Base { execute: ExecuteFn },
    Exec { execute: ExecuteFn<()> },
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

type EngineRequestData = Box<dyn Any + Send>;

#[derive(Debug)]
pub enum EngineRequest {
    Send(EngineRequestData),
    Invoke(
        EngineRequestData,
        tokio::sync::oneshot::Sender<EngineRequestData>,
    ),
}

pub struct ExecuteContext {
    sender: tokio::sync::mpsc::UnboundedSender<EngineRequest>,
    handle: tokio::runtime::Handle,
}

impl ExecuteContext {
    pub fn new(
        sender: tokio::sync::mpsc::UnboundedSender<EngineRequest>,
        handle: tokio::runtime::Handle,
    ) -> Self {
        Self { sender, handle }
    }

    pub fn enter_handle(&self) -> EnterGuard<'_> {
        self.handle.enter()
    }

    pub fn send(&self, data: EngineRequestData) {
        self.sender.send(EngineRequest::Send(data)).unwrap();
    }

    pub async fn invoke(&self, data: EngineRequestData) -> EngineRequestData {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.sender.send(EngineRequest::Invoke(data, tx)).unwrap();
        rx.await.unwrap()
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
  (|$node: ident, $event:ident: $event_type:ident| $($body:tt)*) => {{
    |$node, $event| {
      let $event = $event.downcast_ref::<$event_type>().unwrap();
      $($body)*
    }
  }};
}
