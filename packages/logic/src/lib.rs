use macrograph_core::{exec_fn, DataInput, ExecInput, ExecOutput, NodeSchema, Package, Value};

pub fn create_package() -> Package {
    let mut package = Package::new("logic");

    package.add_schema(NodeSchema::new_base(
        "branch",
        "Branch",
        |node| {
            node.add_exec_input(ExecInput::new("exec", "", &node));
            node.add_exec_output(ExecOutput::new("true", "True"));
            node.add_exec_output(ExecOutput::new("false", "False"));

            node.add_data_input(DataInput::new("condition", "Condition", Value::Bool(false)));
        },
        exec_fn!(|node| {
            if node.get_bool("condition").unwrap() {
                node.find_exec_output("true")
            } else {
                node.find_exec_output("false")
            }
        }),
    ));

    package
}
