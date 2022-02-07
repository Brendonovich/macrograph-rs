use macrograph_package_api::NodeSchemaType;
use serde::Serialize;
use ts_rs::TS;

use crate::schema::NodeSchema;

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

#[derive(TS, Serialize, Debug)]
pub struct RawNodeSchemaRef {
    pub name: String,
    pub package: String,
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

impl From<&NodeSchema> for RawNodeSchemaRef {
    fn from(schema: &NodeSchema) -> Self {
        Self {
            name: schema.name.clone(),
            package: schema.package.clone(),
        }
    }
}
