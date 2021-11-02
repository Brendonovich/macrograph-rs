pub mod engine;
pub mod key_event;
pub mod key;
mod utils;
mod types;

use crate::{core::{package::Package, schema::NodeSchema}, packages::keyboard::key_event::KeyEvent};

use self::engine::{setup_engine};
use crate::core::io::{Output, DataOutput, ExecOutput};

pub fn create_package() -> Package {
  let mut package = Package::new("keyboard");

  for c in 'a'..'z' {
    package.add_schema(NodeSchema::new_event(
      &format!("key_{}", c),
      |node| {
        node.add_exec_output(ExecOutput::new("execute"));
        node.add_data_output(DataOutput::new("key", "".into(), &node));
      },
      |node, event| {
        let event: KeyEvent = serde_json::from_value(event).unwrap();

        let output = node.find_data_output("key").unwrap();
        output.set_value(event.key.to_string().into());

        node.find_exec_output("execute")
      },
    ));
  }

  package.set_engine(setup_engine());

  package
}
