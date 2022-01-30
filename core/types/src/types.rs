use serde_json::Value;
use std::any::Any;
use std::future::Future;
use std::{
    pin::Pin,
    sync::{Arc, Mutex},
};

use crate::{node::Node, Engine, ExecuteContext};

pub type NodeRef = Arc<Node>;
pub type BaseExecuteFn =
    fn(NodeRef, ExecuteContext) -> Pin<Box<dyn Future<Output = Option<&'static str>> + Send>>;
pub type ExecuteFn = fn(NodeRef, ExecuteContext) -> Pin<Box<dyn Future<Output = ()> + Send>>;
pub type BuildFn = fn(NodeRef);
pub type FireFn = fn(NodeRef, &dyn Any) -> Option<&'static str>;

pub type EngineRef = Arc<Engine>;

pub type ArcMutex<T> = Arc<Mutex<T>>;
