pub mod engine;
pub mod key;
pub mod key_event;
mod types;

use macrograph_core::{
    io::{DataOutput, ExecOutput},
    package::Package,
    schema::NodeSchema,
};

use self::engine::setup_engine;
use key_event::KeyEvent;

pub fn create_package() -> Package {
    let mut package = Package::new("keyboard");

    for c in 'a'..'z' {
        package.add_schema(NodeSchema::new_event(
            &format!("key_{}", c),
            &format!("{}", c.to_uppercase()),
            |node| {
                node.add_exec_output(ExecOutput::new("pressed", "Pressed"));
                node.add_exec_output(ExecOutput::new("released", "Released"));

                node.add_data_output(DataOutput::new("shift_pressed", "Shift Pressed", false.into(), &node));
                node.add_data_output(DataOutput::new("ctrl_pressed", "Ctrl Pressed", false.into(), &node));
                node.add_data_output(DataOutput::new("alt_pressed", "Alt Pressed", false.into(), &node));
                node.add_data_output(DataOutput::new("meta_pressed", "Meta Pressed", false.into(), &node));
            },
            |node, event| {
                let event: KeyEvent = serde_json::from_value(event).unwrap();

                node.set_output("shift_pressed", event.shift_pressed.into());
                node.set_output("ctrl_pressed", event.ctrl_pressed.into());
                node.set_output("alt_pressed", event.alt_pressed.into());
                node.set_output("meta_pressed", event.meta_pressed.into());

                node.find_exec_output(if event.pressed { "pressed" } else { "released" })
            },
        ));
    }

    package.set_engine(setup_engine());

    package
}
