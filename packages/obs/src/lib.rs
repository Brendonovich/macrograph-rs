pub mod constants;
pub mod engine;

use constants::{PREVIEW_SCENE_CHANGED, PROGRAM_SCENE_CHANGED, SET_CURRENT_SCENE};
use engine::run;
use macrograph_package_api::{engine::EngineConfig, exec_fn, fire_fn, package::Package, run_fn};

#[no_mangle]
pub fn create_package() -> Package {
    let mut package = Package::new("OBS");
    package.set_engine(EngineConfig {
        run: run_fn!(run),
        state: None,
    });

    package.add_exec_schema(
        SET_CURRENT_SCENE,
        |s| s.data_input("Scene", "Main".into()),
        exec_fn!(|io, ctx| async {
            let scene = io.get_string("Scene").unwrap();
        }),
    );

    [PREVIEW_SCENE_CHANGED, PROGRAM_SCENE_CHANGED]
        .iter()
        .for_each(|e| {
            package.add_event_schema(
                e,
                |s| {
                    s.exec_output("");
                    s.data_output("Scene Name", "".into())
                },
                fire_fn!(|io, event: String| {
                    io.set_string("Scene Name", &event);

                    Some("")
                }),
            );
        });

    package
}
