use std::{
    ops::Deref,
    sync::{Arc, Mutex},
};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::{
    primitive::Primitive,
    types::{FromType, IntoType, ListType, PrimitiveType},
};

#[derive(TS, Serialize, Deserialize, Debug, Clone)]
#[ts(export)]
#[serde(rename_all = "camelCase", tag = "type", content = "value")]
pub enum ListItem {
    Primitive(Primitive),
}

impl From<Primitive> for ListItem {
    fn from(primitive: Primitive) -> Self {
        ListItem::Primitive(primitive)
    }
}

#[derive(TS, Serialize, Deserialize, Debug, Clone)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct List {
    pub r#type: ListType,
    pub values: Arc<Mutex<Vec<ListItem>>>,
}

impl List {
    pub fn new(r#type: ListType) -> Self {
        List {
            r#type,
            values: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl Deref for List {
    type Target = Arc<Mutex<Vec<ListItem>>>;

    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl From<ListType> for List {
    fn from(list_type: ListType) -> Self {
        List::new(list_type)
    }
}

impl<T: IntoType<PrimitiveType> + Into<Primitive>> From<Vec<T>> for List {
    fn from(values: Vec<T>) -> Self {
        let values = values.into_iter().map(|i| i.into().into());
        List {
            r#type: ListType::Primitive(T::into_type()),
            values: Arc::new(Mutex::new(values.collect())),
        }
    }
}
