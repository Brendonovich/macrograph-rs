use macrograph_package_api::{package::Package, exec_fn};

#[no_mangle]
pub fn create_package() -> Package {
    let mut pkg = Package::new("Logic");
    
    pkg.add_base_schema(
        "Branch",
        |s| {
            s.exec_input("");
            s.data_input("Condition", false.into());
            s.exec_output("True");
            s.exec_output("False");
        },
        exec_fn!(|io, _ctx| {
            Some(if io.get_bool("Condition").unwrap() {
                "True"
            } else {
                "False"
            })
        }),
    );
    
    pkg
}
