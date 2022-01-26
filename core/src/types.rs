use futures::Future;
use serde_json::Value;
use std::{
    pin::Pin,
    sync::{Arc, Mutex},
};

use crate::{Engine, ExecOutput, ExecuteContext, Node, NodeSchema};

pub type ExecuteOutput = Option<Arc<ExecOutput>>;
pub type NodeRef = Arc<Node>;
pub type SyncExecuteFn = fn(NodeRef, ExecuteContext) -> ExecuteOutput;
pub type ExecuteFn =
    fn(NodeRef, ExecuteContext) -> Pin<Box<dyn Future<Output = ExecuteOutput> + Send>>;
pub type BuildFn = fn(NodeRef);
pub type FireFn = fn(NodeRef, Value) -> ExecuteOutput;

pub type NodeSchemaRef = Arc<NodeSchema>;
pub type EngineRef = Arc<tokio::sync::Mutex<Engine>>;

pub type ArcMutex<T> = Arc<Mutex<T>>;
