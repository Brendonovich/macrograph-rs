use macrograph_core_types::{exec_fn, Package};

#[no_mangle]
pub fn create_package(package: &mut Package) {
    package.set_name("Utils");

    package.add_exec_schema(
        "Print",
        |node| {
            node.add_data_input("Value", "".into());
        },
        exec_fn!(|node, _ctx| async {
            println!("Print: {}", node.get_string("Value").unwrap());
        }),
    );
}
