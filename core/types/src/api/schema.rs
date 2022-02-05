use serde::Serialize;
use ts_rs::TS;

use crate::schema::{NodeSchema, NodeSchemaType};

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
    pub name: String,
    pub package: String,
    #[serde(rename = "type")]
    pub t: RawNodeSchemaType,
}

impl From<&NodeSchema> for RawNodeSchema {
    fn from(schema: &NodeSchema) -> Self {
        Self {
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
