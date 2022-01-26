pub mod engine;

use engine::State;
use macrograph_core::{exec_fn, package::Package, DataInput, NodeSchema};

use self::engine::setup_engine;

pub fn create_package() -> Package {
    let mut package = Package::new("obs");

    package.set_engine(setup_engine());

    package.add_schema(NodeSchema::new_exec(
        "set_current_scene",
        "OBS Set Current Scene",
        |n| {
            n.add_data_input(DataInput::new("scene", "Scene", "Main".into()));
        },
        exec_fn!(|n, ctx| async {
            let scene = n.get_string("scene").unwrap();
            let engine = ctx.engine.unwrap();
            let mut engine = engine.lock().await;

            let state: &State = engine.state();
            state
                .client
                .as_ref()
                .unwrap()
                .scenes()
                .set_current_program_scene(&scene)
                .await;

            n.find_exec_output("execute")
        }),
    ));

    package
}
