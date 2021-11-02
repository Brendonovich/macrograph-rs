use crate::{
    core::{
        package::Package,
        schema::NodeSchema,
        value::{Value},
    },
    exec_fn,
};
use crate::core::io::DataInput;

pub fn create_package() -> Package {
    let mut package = Package::new("utils");

    package.add_schema(NodeSchema::new_exec(
        "print",
        |node| {
            node.add_data_input(DataInput::new("value", Value::String("".into())));
        },
        exec_fn!(|node| {
            let value = node.find_data_input("value").unwrap().get_value();

            println!("{}", value);

            None
        }),
    ));

    package
}
