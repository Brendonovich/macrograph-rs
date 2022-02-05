use macrograph_core_types::{exec_fn, Package};

#[no_mangle]
pub fn create_package() -> Package {
    let mut package = Package::new("Utils");

    package.add_exec_schema(
        "Print",
        |s| {
            s.data_input("Value", "".into());
        },
        exec_fn!(|n, _ctx| async {
            println!("Print: {}", n.get_string("Value").unwrap());
        }),
    );
    
    package
}
