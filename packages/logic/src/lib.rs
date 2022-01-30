use macrograph_core_types::{exec_fn, Package};

#[no_mangle]
pub fn create_package(pkg: &mut Package) {
    pkg.set_name("Logic");
    pkg.add_base_schema(
        "Branch",
        |n| {
            n.add_exec_input("");
            n.add_data_input("Condition", false.into());

            n.add_exec_output("True");
            n.add_exec_output("False");
        },
        exec_fn!(|n, _ctx| async {
            Some(if n.get_bool("Condition").unwrap() {
                "True"
            } else {
                "False"
            })
        }),
    )
}
