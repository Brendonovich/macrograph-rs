use serde::Serialize;
use ts_rs::TS;

use crate::Package;

use super::schema::RawNodeSchema;

#[derive(TS, Serialize, Debug)]
#[ts(export)]
#[serde(rename = "Package")]
pub struct RawPackage {
    pub name: String,
    pub schemas: Vec<RawNodeSchema>,
}

impl From<&Package> for RawPackage {
    fn from(package: &Package) -> Self {
        Self {
            name: package.name.clone(),
            schemas: package.schemas.iter().map(|s| (&**s).into()).collect(),
        }
    }
}
