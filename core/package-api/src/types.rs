use std::pin::Pin;
use std::{any::Any, future::Future};

use crate::{BuildSchema, ExecuteContext, IOProxy};

pub type BuildFn = fn(&mut BuildSchema);
pub type SyncExecuteFn<T = Option<&'static str>> = fn(&mut IOProxy, ExecuteContext) -> T;
pub type AsyncExecuteFn<T = Option<&'static str>> =
    fn(&mut IOProxy, ExecuteContext) -> Pin<Box<dyn Future<Output = T> + Send + '_>>;
pub type FireFn = fn(&mut IOProxy, &dyn Any) -> Option<&'static str>;
