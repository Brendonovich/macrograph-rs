use macrograph::{exec_fn, DataInput, NodeSchema, Package, Value};

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
