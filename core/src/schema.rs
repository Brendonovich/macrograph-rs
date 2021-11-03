use crate::{types::*, ExecInput, ExecOutput};

pub struct ExecuteContext {
    pub engine: Option<EngineRef>,
}

pub enum ExecuteFn {
    Sync(SyncExecuteFn),
    Async(AsyncExecuteFn),
}

impl From<SyncExecuteFn> for ExecuteFn {
    fn from(func: SyncExecuteFn) -> Self {
        Self::Sync(func)
    }
}

impl From<AsyncExecuteFn> for ExecuteFn {
    fn from(func: AsyncExecuteFn) -> Self {
        Self::Async(func)
    }
}

pub enum NodeSchema {
    Exec(ExecNodeSchema),
    Event(EventNodeSchema),
}

impl NodeSchema {
    pub fn new_exec(id: &str, build: BuildFn, execute: ExecuteFn) -> Self {
        NodeSchema::Exec(ExecNodeSchema {
            id: id.into(),
            build,
            execute,
            package: String::new(),
        })
    }

    pub fn new_event(id: &str, build: BuildFn, fire: FireFn) -> Self {
        NodeSchema::Event(EventNodeSchema {
            id: id.into(),
            build,
            fire,
            package: String::new(),
        })
    }

    pub fn get_id(&self) -> &str {
        match self {
            NodeSchema::Exec(schema) => &schema.id,
            NodeSchema::Event(schema) => &schema.id,
        }
    }

    pub fn build(&self, node: NodeRef) {
        match self {
            NodeSchema::Exec(schema) => {
                node.add_exec_input(ExecInput::new("execute", &node));
                node.add_exec_output(ExecOutput::new("execute"));
                (schema.build)(node)
            }
            NodeSchema::Event(schema) => (schema.build)(node),
        }
    }
}

pub struct ExecNodeSchema {
    pub id: String,
    pub package: String,
    pub build: BuildFn,
    pub execute: ExecuteFn,
}

pub struct EventNodeSchema {
    pub id: String,
    pub package: String,
    pub build: BuildFn,
    pub fire: FireFn,
}

#[macro_export]
macro_rules! exec_fn {
  (|$t:ident, $core:ident| $body:block) => {{
    $crate::schema::ExecuteFn::Sync(|$t, $core| $body)
  }};
  (|$t:ident| $body:block) => {{
    $crate::schema::ExecuteFn::Sync(|$t, _| $body)
  }};
  (|$t:ident, $core:ident| async $($body:tt)*) => {{
    $crate::schema::ExecuteFn::Async(|$t, $core| Box::pin(async move {
      async $($body)*.await
    }))
  }};
}
