use std::{
    ops::Deref,
    sync::{Arc, Weak},
};
use tokio::sync::Mutex;
use weak_table::PtrWeakHashSet;

use crate::{
    node::{BuildSchema, Node},
    types::*,
    Engine,
};

pub struct ExecuteContext {
    pub engine: Option<Arc<Engine>>,
    pub handle: tokio::runtime::Handle,
}

pub enum NodeSchemaType {
    Base { execute: BaseExecuteFn },
    Exec { execute: ExecuteFn },
    Event { fire: FireFn },
}

pub struct NodeSchema {
    pub name: String,
    pub package: String,
    pub build: BuildFn,
    pub instances: Mutex<PtrWeakHashSet<Weak<Node>>>,
    pub inner: NodeSchemaType,
}

impl Deref for NodeSchema {
    type Target = NodeSchemaType;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl NodeSchema {
    pub fn new_exec(name: &str, build: BuildFn, execute: ExecuteFn) -> Self {
        Self {
            name: name.into(),
            build,
            package: String::new(),
            inner: NodeSchemaType::Exec { execute },
            instances: Mutex::new(PtrWeakHashSet::new()),
        }
    }

    pub fn new_base(name: &str, build: BuildFn, execute: BaseExecuteFn) -> Self {
        Self {
            name: name.into(),
            build,
            package: String::new(),
            inner: NodeSchemaType::Base { execute },
            instances: Mutex::new(PtrWeakHashSet::new()),
        }
    }

    pub fn new_event(name: &str, build: BuildFn, fire: FireFn) -> Self {
        Self {
            name: name.into(),
            build,
            package: String::new(),
            inner: NodeSchemaType::Event { fire },
            instances: Mutex::new(PtrWeakHashSet::new()),
        }
    }

    pub fn build(&self, schema: &mut BuildSchema) {
        use NodeSchemaType::*;

        match **self {
            Exec { .. } => {
                schema.exec_input("");
                schema.exec_output("");
            }
            _ => {}
        }

        (self.build)(schema);
    }
}

#[macro_export]
macro_rules! exec_fn {
  (|$t:ident, $ctx:ident| async $($body:tt)*) => {{
    |$t, $ctx|
        Box::pin(async move {
            let _guard = $ctx.handle.enter();
            async $($body)*.await
       })
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
