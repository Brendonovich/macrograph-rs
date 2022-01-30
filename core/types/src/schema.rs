use std::{
    ops::Deref,
    sync::{Arc, Weak},
};
use tokio::sync::Mutex;

use weak_table::WeakHashSet;

use crate::{node::Node, types::*, Engine};

pub struct ExecuteContext {
    pub engine: Option<Arc<Engine>>,
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
    pub instances: Mutex<WeakHashSet<Weak<Node>>>,
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
            instances: Mutex::new(WeakHashSet::new()),
        }
    }

    pub fn new_base(name: &str, build: BuildFn, execute: BaseExecuteFn) -> Self {
        Self {
            name: name.into(),
            build,
            package: String::new(),
            inner: NodeSchemaType::Base { execute },
            instances: Mutex::new(WeakHashSet::new()),
        }
    }

    pub fn new_event(name: &str, build: BuildFn, fire: FireFn) -> Self {
        Self {
            name: name.into(),
            build,
            package: String::new(),
            inner: NodeSchemaType::Event { fire },
            instances: Mutex::new(WeakHashSet::new()),
        }
    }

    pub fn build(&self, node: NodeRef) {
        use NodeSchemaType::*;

        match **self {
            Exec { .. } => {
                node.add_exec_input("");
                node.add_exec_output("");
            }
            _ => {}
        }

        (self.build)(node);
    }
}

#[macro_export]
macro_rules! exec_fn {
  (|$t:ident, $ctx:ident| async $($body:tt)*) => {{
    |$t, $ctx| Box::pin(async move {
      let _handle = if let Some(_engine) = &$ctx.engine {
        let handle = _engine.runtime.handle().clone();
        Some(handle)
      } else {
        None
      };

      if let Some(_handle) = _handle {
        let _guard = _handle.enter();

        let _res = async $($body)*.await;
        
        _res
      } else {
        async $($body)*.await
      }
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
