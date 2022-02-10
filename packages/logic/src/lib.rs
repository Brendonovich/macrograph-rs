use macrograph_package_api::{exec_fn, package::Package};

#[no_mangle]
pub fn create_package() -> Package {
    let mut pkg = Package::new("Logic");

    pkg.add_base_schema(
        "Branch",
        |s| {
            s.exec_input("");
            s.bool_input("Condition");
            s.exec_output("True");
            s.exec_output("False");
        },
        exec_fn!(|io, _ctx| {
            io.get_bool("Condition")
                .map(|c| if c { "True" } else { "False" })
        }),
    );

    pkg
}
