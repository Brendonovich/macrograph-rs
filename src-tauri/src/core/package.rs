use std::{
  sync::{Arc, Mutex},
};

use super::{engine::{Engine}, types::{EngineRef, NodeSchemaRef}};
use crate::core::schema::NodeSchema;

pub struct Package {
  pub name: String,
  pub(crate) schemas: Vec<NodeSchemaRef>,
  pub engine: Option<EngineRef>,
}

impl Package {
  pub fn new(name: &str) -> Self {
    Self {
      name: name.into(),
      schemas: vec![],
      engine: None,
    }
  }

  pub fn add_schema(&mut self, mut schema: NodeSchema) {
    match schema {
      NodeSchema::Event(ref mut schema) => schema.package = self.name.to_string(),
      NodeSchema::Exec(ref mut schema) => schema.package = self.name.to_string(),
    };

    self.schemas.push(Arc::new(schema));
  }

  pub fn set_engine(&mut self, engine: Engine) {
    self.engine = Some(Arc::new(Mutex::new(engine)));
  }
  
  pub fn schema(&self, name: &str) -> Option<&NodeSchemaRef> {
    self.schemas.iter().find(|s| s.get_id() == name)
  }
}

#[cfg(test)]
mod test {
  use crate::{core::schema::NodeSchema, exec_fn};

  use super::Package;

  #[test]
  fn adds_schema_to_package() {
    let mut package = Package::new("Test Package");

    let schema = NodeSchema::new_exec("test", |_n| {}, exec_fn!(|_n, _c| { None }));

    package.add_schema(schema);

    assert!(package.name == "Test Package")
  }
}
