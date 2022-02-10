use macrograph_package_api::{exec_fn, package::Package};

#[no_mangle]
pub fn create_package() -> Package {
    let mut package = Package::new("Utils");

    package.add_exec_schema(
        "Print",
        |s| {
            s.string_input("Value");
        },
        exec_fn!(|n, _ctx| {
            n.get_string("Value").map(|v| println!("Print: {}", v));
        }),
    );

    package.add_exec_schema(
        "Print List",
        |io| {
            io.list_input::<String>("Value");
        },
        exec_fn!(|io, _ctx| {
            io.get_list::<String>("Value")
                .map(|v| println!("Print: {:?}", v.values.lock().unwrap()));
        }),
    );

    package
}
