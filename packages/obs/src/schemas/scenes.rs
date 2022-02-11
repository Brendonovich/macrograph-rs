use crate::engine::Request;
use crate::types::{SceneCreated, SceneRemoved};
use crate::{constants::*, types::SceneNameChanged};
use macrograph_package_api::{exec_fn, fire_fn, package::Package};

pub fn create_scene_schemas(package: &mut Package) {
    package.add_exec_schema(
        SET_CURRENT_SCENE,
        |s| {
            s.string_input("Scene");
            s.bool_input("Preview");
        },
        exec_fn!(|io, ctx| {
            io.get_string("Scene")
                .zip(io.get_bool("Preview"))
                .map(|(scene_name, preview)| {
                    ctx.send(Request::SetCurrentScene {
                        scene_name,
                        preview,
                    })
                });
        }),
    );

    package.add_exec_schema(
        GET_CURRENT_SCENE,
        |s| {
            s.string_output("Scene");
        },
        exec_fn!(|io, ctx| async {
            ctx.invoke(Request::GetCurrentScene)
                .await
                .map(|scene_name: String| {
                    println!("scene name {}", scene_name);
                    io.set_string("Scene", scene_name);
                });
        }),
    );

    [PREVIEW_SCENE_CHANGED, PROGRAM_SCENE_CHANGED]
        .iter()
        .for_each(|e| {
            package.add_event_schema(
                e,
                |s| {
                    s.exec_output("");
                    s.string_output("Scene")
                },
                fire_fn!(|io, scene_name: &String| {
                    io.set_string("Scene", scene_name.to_string());

                    Some("")
                }),
            );
        });

    package.add_exec_schema(
        CREATE_SCENE,
        |io| io.string_input("Scene"),
        exec_fn!(|io, ctx| {
            io.get_string("Scene")
                .map(|scene_name| ctx.send(Request::CreateScene { scene_name }));
        }),
    );

    package.add_exec_schema(
        REMOVE_SCENE,
        |io| io.string_output("Scene"),
        exec_fn!(|io, ctx| {
            io.get_string("Scene")
                .map(|scene_name| ctx.send(Request::RemoveScene { scene_name }));
        }),
    );

    package.add_exec_schema(
        SET_SCENE_NAME,
        |io| {
            io.string_output("Scene");
            io.string_output("New");
        },
        exec_fn!(|io, ctx| {
            match (io.get_string("Scene"), io.get_string("New")) {
                (Some(scene_name), Some(new_name)) => ctx.send(Request::SetSceneName {
                    scene_name,
                    new_name,
                }),
                _ => {}
            };
        }),
    );

    package.add_exec_schema(
        GET_SCENE_LIST,
        |io| {
            io.list_output::<String>("Scenes");
        },
        exec_fn!(|io, ctx| async {
            ctx.invoke(Request::GetSceneList)
                .await
                .map(|v: Vec<String>| io.set_list("Scenes", v));
        }),
    );

    package.add_event_schema(
        SCENE_CREATED,
        |s| {
            s.exec_output("");
            s.string_output("Scene");
            s.bool_output("Is Group");
        },
        fire_fn!(|io, event: &SceneCreated| {
            io.set_string("Scene", event.scene_name.to_string());
            io.set_bool("Is Group", event.is_group);

            Some("")
        }),
    );

    package.add_event_schema(
        SCENE_REMOVED,
        |s| {
            s.exec_output("");
            s.string_output("Scene");
            s.bool_output("Is Group");
        },
        fire_fn!(|io, event: &SceneRemoved| {
            io.set_string("Scene", event.scene_name.to_string());
            io.set_bool("Is Group", event.is_group);

            Some("")
        }),
    );

    package.add_event_schema(
        SCENE_NAME_CHANGED,
        |s| {
            s.exec_output("");
            s.string_output("Scene");
            s.string_output("Old Scene");
        },
        fire_fn!(|io, data: &SceneNameChanged| {
            io.set_string("Scene", data.scene_name.to_string());
            io.set_string("Old Scene", data.old_scene_name.to_string());

            Some("")
        }),
    );
}
