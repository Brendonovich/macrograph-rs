use std::any::Any;
use std::future::Future;
use std::{
    pin::Pin,
    sync::{Arc, Mutex},
};

use crate::node::{BuildSchema, IOData};
use crate::{node::Node, Engine, ExecuteContext};

pub type NodeRef = Arc<Node>;
pub type BaseExecuteFn =
    fn(&mut IOData, ExecuteContext) -> Pin<Box<dyn Future<Output = Option<&'static str>> + Send + '_>>;
pub type ExecuteFn = fn(&mut IOData, ExecuteContext) -> Pin<Box<dyn Future<Output = ()> + Send + '_>>;
pub type BuildFn = fn(&mut BuildSchema);
pub type FireFn = fn(&mut IOData, &dyn Any) -> Option<&'static str>;

pub type EngineRef = Arc<Engine>;

pub type ArcMutex<T> = Arc<Mutex<T>>;
