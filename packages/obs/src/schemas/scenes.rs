use crate::engine::Request;
use crate::types::{SceneCreated, SceneRemoved};
use crate::{constants::*, types::SceneNameChanged};
use macrograph_package_api::value::types::PrimitiveType;
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

    [PREVIEW_SCENE_CHANGED, PROGRAM_SCENE_CHANGED]
        .iter()
        .for_each(|e| {
            package.add_event_schema(
                e,
                |s| {
                    s.exec_output("");
                    s.string_output("Scene Name")
                },
                fire_fn!(|io, event: String| {
                    io.set_string("Scene Name", &event);

                    Some("")
                }),
            );
        });

    package.add_exec_schema(
        CREATE_SCENE,
        |io| io.string_input("Scene Name"),
        exec_fn!(|io, ctx| {
            io.get_string("Scene Name")
                .map(|scene_name| ctx.send(Request::CreateScene { scene_name }));
        }),
    );

    package.add_exec_schema(
        REMOVE_SCENE,
        |io| io.string_output("Scene Name"),
        exec_fn!(|io, ctx| {
            io.get_string("Scene Name")
                .map(|scene_name| ctx.send(Request::RemoveScene { scene_name }));
        }),
    );

    package.add_exec_schema(
        SET_SCENE_NAME,
        |io| {
            io.string_output("Scene Name");
            io.string_output("New Name");
        },
        exec_fn!(|io, ctx| {
            io.get_string("Scene Name")
                .zip(io.get_string("New Name"))
                .map(|(scene_name, new_name)| {
                    ctx.send(Request::SetSceneName {
                        scene_name,
                        new_name,
                    })
                });
        }),
    );

    package.add_exec_schema(
        GET_SCENE_LIST,
        |io| {
            io.list_output("Scenes", PrimitiveType::String.into());
        },
        exec_fn!(|io, _ctx| {
            io.set_list("Scenes", vec!["1", "2", "3"]);
        }),
    );

    package.add_event_schema(
        SCENE_CREATED,
        |s| {
            s.exec_output("");
            s.string_output("Scene Name");
            s.bool_output("Is Group");
        },
        fire_fn!(|io, event: SceneCreated| {
            io.set_string("Scene Name", &event.scene_name);
            io.set_bool("Is Group", event.is_group);

            Some("")
        }),
    );

    package.add_event_schema(
        SCENE_REMOVED,
        |s| {
            s.exec_output("");
            s.string_output("Scene Name");
            s.bool_output("Is Group");
        },
        fire_fn!(|io, event: SceneRemoved| {
            io.set_string("Scene Name", &event.scene_name);
            io.set_bool("Is Group", event.is_group);

            Some("")
        }),
    );

    package.add_event_schema(
        SCENE_NAME_CHANGED,
        |s| {
            s.exec_output("");
            s.string_output("Scene Name");
            s.string_output("Old Scene Name");
        },
        fire_fn!(|io, data: SceneNameChanged| {
            io.set_string("Scene Name", &data.scene_name);
            io.set_string("Old Scene Name", &data.old_scene_name);

            Some("")
        }),
    );
}
