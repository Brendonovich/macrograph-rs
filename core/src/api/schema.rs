use serde::Serialize;
use ts_rs::TS;

use crate::{schema::NodeSchemaType, NodeSchema};

#[derive(TS, Serialize, Debug)]
#[ts(export)]
#[serde(rename = "NodeSchemaType")]
pub enum RawNodeSchemaType {
    Base,
    Exec,
    Event,
}

#[derive(TS, Serialize, Debug)]
#[ts(export)]
#[serde(rename = "NodeSchema")]
pub struct RawNodeSchema {
    pub id: String,
    pub name: String,
    pub package: String,
    #[serde(rename = "type")]
    pub t: RawNodeSchemaType,
}

impl From<&NodeSchema> for RawNodeSchema {
    fn from(schema: &NodeSchema) -> Self {
        Self {
            id: schema.id.clone(),
            name: schema.name.clone(),
            package: schema.package.clone(),
            t: match schema.inner {
                NodeSchemaType::Base { .. } => RawNodeSchemaType::Base,
                NodeSchemaType::Exec { .. } => RawNodeSchemaType::Exec,
                NodeSchemaType::Event { .. } => RawNodeSchemaType::Event,
            },
        }
    }
}
