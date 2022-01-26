use std::{ops::Deref, sync::Weak};
use tokio::sync::Mutex;

use weak_table::WeakHashSet;

use crate::{types::*, ExecInput, ExecOutput, Node};

pub struct ExecuteContext {
    pub engine: Option<EngineRef>,
}

pub enum NodeSchemaType {
    Base { execute: ExecuteFn },
    Exec { execute: ExecuteFn },
    Event { fire: FireFn },
}

pub struct NodeSchema {
    pub id: String,
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
    pub fn new_exec(id: &str, name: &str, build: BuildFn, execute: ExecuteFn) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            build,
            package: String::new(),
            inner: NodeSchemaType::Exec { execute },
            instances: Mutex::new(WeakHashSet::new()),
        }
    }

    pub fn new_base(id: &str, name: &str, build: BuildFn, execute: ExecuteFn) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            build,
            package: String::new(),
            inner: NodeSchemaType::Base { execute },
            instances: Mutex::new(WeakHashSet::new()),
        }
    }

    pub fn new_event(id: &str, name: &str, build: BuildFn, fire: FireFn) -> Self {
        Self {
            id: id.into(),
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
                node.add_exec_input(ExecInput::new("execute", "", &node));
                node.add_exec_output(ExecOutput::new("execute", ""));
            }
            _ => {}
        }

        (self.build)(node);
    }
}

#[macro_export]
macro_rules! exec_fn {
  (|$t:ident, $core:ident| async $($body:tt)*) => {{
    |$t, $core| Box::pin(async move {
      async $($body)*.await
    })
  }};
  (|$t:ident| async $($body:tt)*) => {{
    |$t, _| Box::pin(async move {
      async $($body)*.await
    })
  }};
  (|$t:ident, $core:ident|  $($body:tt)*) => {{
    |$t, $core| Box::pin(async move {
      $($body)*
    })
  }};
  (|$t:ident| $($body:tt)*) => {{
    |$t, _| Box::pin(async move {
      $($body)*
    })
  }};
}
