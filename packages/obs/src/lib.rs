pub mod engine;

use engine::{setup_engine, State};
use macrograph_core_types::{exec_fn, package::Package};

#[no_mangle]
pub fn create_package(package: &mut Package) {
    package.set_name("obs");
    package.set_engine(setup_engine());

    package.add_exec_schema(
        "Set Current Scene",
        |n| n.add_data_input("Scene", "Main".into()),
        exec_fn!(|n, ctx| async {
            let scene = n.get_string("Scene").unwrap();
            let engine = ctx.engine.unwrap();
            let state = engine.state::<State>().await;
            state
                .client
                .as_ref()
                .unwrap()
                .scenes()
                .set_current_program_scene(&scene)
                .await
                .unwrap();
        }),
    );
}
