use macrograph::{exec_fn, DataOutput, NodeSchema, Package};

use engine::{setup_engine, EngineState};

mod engine;

pub fn create_package() -> Package {
    let mut package = Package::new("midi");

    package.add_schema(NodeSchema::new_exec(
        "first_output",
        |node| {
            node.add_data_output(DataOutput::new("output", "".into(), &node));
        },
        exec_fn!(|node, ctx| {
            let engine = ctx.engine.unwrap();
            let mut engine = engine.lock().unwrap();
            let state: &EngineState = engine.state();

            let ports = state.output.ports();

            let name: String = match ports.get(0) {
                Some(p) => state.output.port_name(p).unwrap_or("".into()),
                None => "".into(),
            };

            node.find_data_output("output")
                .unwrap()
                .set_value(name.into());

            node.find_exec_output("execute")
        }),
    ));

    package.set_engine(setup_engine());

    package
}
