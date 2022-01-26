use macrograph_core::{exec_fn, DataInput, NodeSchema, Package, Value};

pub fn create_package() -> Package {
    let mut package = Package::new("utils");

    package.add_schema(NodeSchema::new_exec(
        "print",
        "Print",
        |node| {
            node.add_data_input(DataInput::new("value", "Value", "".into()));
        },
        exec_fn!(|node| {
            println!("Print: {}", node.get_string("value").unwrap());

            None
        }),
    ));

    package
}
