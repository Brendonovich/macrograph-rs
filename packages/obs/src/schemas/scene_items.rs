use crate::constants::*;
use crate::engine::Request;
use crate::types::{
    SceneItemCreated, SceneItemEnableChanged, SceneItemLockChanged, SceneItemRemoved,
    SceneItemSelected,
};
use macrograph_package_api::{exec_fn, fire_fn, package::Package};

pub fn create_scene_item_schemas(package: &mut Package) {
    package.add_exec_schema(
        GET_SCENE_ITEM_ID,
        |io| {
            io.string_input("Scene");
            io.string_input("Source");
            io.int_output("Item ID");
        },
        exec_fn!(|io, ctx| async {
            match (io.get_string("Scene"), io.get_string("Source")) {
                (Some(scene_name), Some(source_name)) => {
                    ctx.invoke(Request::GetSceneItemID {
                        scene_name,
                        source_name,
                    })
                    .await
                    .map(|id: i32| {
                        io.set_int("Item ID", id);
                    });
                }
                _ => {}
            };
        }),
    );

    package.add_exec_schema(
        CREATE_SCENE_ITEM,
        |io| {
            io.string_input("Scene");
            io.string_input("Source");
            io.bool_input("Enabled");
        },
        exec_fn!(|io, ctx| {
            match (
                io.get_string("Scene"),
                io.get_string("Source"),
                io.get_bool("Enabled"),
            ) {
                (Some(scene_name), Some(source_name), Some(enabled)) => {
                    ctx.send(Request::CreateSceneItem {
                        scene_name,
                        source_name,
                        enabled,
                    })
                }
                _ => {}
            };
        }),
    );

    package.add_exec_schema(
        REMOVE_SCENE_ITEM,
        |io| {
            io.string_input("Scene");
            io.int_input("Item ID");
        },
        exec_fn!(|io, ctx| {
            io.get_string("Scene")
                .zip(io.get_int("Item ID"))
                .map(|(scene_name, item_id)| {
                    ctx.send(Request::RemoveSceneItem {
                        scene_name,
                        item_id: item_id as i64,
                    })
                });
        }),
    );

    package.add_exec_schema(
        GET_SCENE_ITEM_ENABLED,
        |io| {
            io.string_input("Scene");
            io.int_input("Item ID");
            io.bool_output("Enabled");
        },
        exec_fn!(|io, ctx| async {
            match (io.get_string("Scene"), io.get_int("Item ID")) {
                (Some(scene_name), Some(item_id)) => {
                    ctx.invoke(Request::GetSceneItemEnabled {
                        scene_name,
                        item_id: item_id as i64,
                    })
                    .await
                    .map(|enabled: bool| {
                        io.set_bool("Enabled", enabled);
                    });
                }
                _ => {}
            };
        }),
    );

    package.add_exec_schema(
        SET_SCENE_ITEM_ENABLED,
        |io| {
            io.string_input("Scene");
            io.int_input("Item ID");
            io.bool_input("Enabled");
        },
        exec_fn!(|io, ctx| {
            match (
                io.get_string("Scene"),
                io.get_int("Item ID"),
                io.get_bool("Enabled"),
            ) {
                (Some(scene_name), Some(item_id), Some(enabled)) => {
                    ctx.send(Request::SetSceneItemEnabled {
                        scene_name,
                        item_id: item_id as i64,
                        enabled,
                    })
                }
                _ => {}
            };
        }),
    );

    package.add_exec_schema(
        GET_SCENE_ITEM_LOCKED,
        |io| {
            io.string_input("Scene");
            io.int_input("Item ID");
            io.bool_output("Locked");
        },
        exec_fn!(|io, ctx| {
            match (io.get_string("Scene"), io.get_int("Item ID")) {
                (Some(scene_name), Some(item_id)) => ctx.send(Request::GetSceneItemLocked {
                    scene_name,
                    item_id: item_id as i64,
                }),
                _ => {}
            };
        }),
    );

    package.add_exec_schema(
        SET_SCENE_ITEM_LOCKED,
        |io| {
            io.string_input("Scene");
            io.int_input("Item ID");
            io.bool_input("Locked");
        },
        exec_fn!(|io, ctx| {
            match (
                io.get_string("Scene"),
                io.get_int("Item ID"),
                io.get_bool("Locked"),
            ) {
                (Some(scene_name), Some(item_id), Some(locked)) => {
                    ctx.send(Request::SetSceneItemLocked {
                        scene_name,
                        item_id: item_id as i64,
                        locked,
                    })
                }
                _ => {}
            };
        }),
    );

    package.add_exec_schema(
        GET_SCENE_ITEM_INDEX,
        |io| {
            io.string_input("Scene");
            io.int_input("Item ID");
            io.int_output("Index");
        },
        exec_fn!(|io, ctx| {
            io.get_string("Scene")
                .zip(io.get_int("Item ID"))
                .map(|(scene_name, item_id)| {
                    ctx.send(Request::GetSceneItemIndex {
                        scene_name,
                        item_id: item_id as i64,
                    })
                });
        }),
    );

    package.add_exec_schema(
        SET_SCENE_ITEM_INDEX,
        |io| {
            io.string_input("Scene");
            io.int_input("Item ID");
            io.int_input("Index");
        },
        exec_fn!(|io, ctx| {
            match (
                io.get_string("Scene"),
                io.get_int("Item ID"),
                io.get_int("Index"),
            ) {
                (Some(scene_name), Some(item_id), Some(index)) => {
                    ctx.send(Request::SetSceneItemIndex {
                        scene_name,
                        item_id: item_id as i64,
                        index: index as u32,
                    })
                }
                _ => {}
            };
        }),
    );

    package.add_event_schema(
        SCENE_ITEM_CREATED,
        |io| {
            io.exec_output("");
            io.string_output("Scene");
            io.string_output("Source");
            io.int_output("Item ID");
            io.int_output("Item Index");
        },
        fire_fn!(|io, data: &SceneItemCreated| {
            io.set_string("Scene", data.scene_name.to_string());
            io.set_string("Source", data.source_name.to_string());
            io.set_int("Item ID", data.item_id as i32);
            io.set_int("Item Index", data.item_index as i32);

            Some("")
        }),
    );

    package.add_event_schema(
        SCENE_ITEM_REMOVED,
        |io| {
            io.exec_output("");
            io.string_output("Scene");
            io.string_output("Source");
            io.int_output("Item ID");
        },
        fire_fn!(|io, data: &SceneItemRemoved| {
            io.set_string("Scene", data.scene_name.to_string());
            io.set_string("Source", data.input_name.to_string());
            io.set_int("Item ID", data.item_id as i32);

            Some("")
        }),
    );

    package.add_event_schema(
        SCENE_ITEM_ENABLED_CHANGED,
        |io| {
            io.exec_output("");
            io.string_output("Scene");
            io.int_output("Item ID");
            io.bool_output("Enabled");
        },
        fire_fn!(|io, data: &SceneItemEnableChanged| {
            io.set_string("Scene", data.scene_name.to_string());
            io.set_int("Item ID", data.item_id as i32);
            io.set_bool("Enabled", data.enabled);

            Some("")
        }),
    );

    package.add_event_schema(
        SCENE_ITEM_LOCK_CHANGED,
        |io| {
            io.exec_output("");
            io.string_output("Scene");
            io.int_output("Item ID");
            io.bool_output("Locked");
        },
        fire_fn!(|io, data: &SceneItemLockChanged| {
            io.set_string("Scene", data.scene_name.to_string());
            io.set_int("Item ID", data.item_id as i32);
            io.set_bool("Locked", data.locked);

            Some("")
        }),
    );

    package.add_event_schema(
        SCENE_ITEM_SELECTED,
        |io| {
            io.exec_output("");
            io.string_output("Scene");
            io.int_output("Item ID");
        },
        fire_fn!(|io, data: &SceneItemSelected| {
            io.set_string("Scene", data.scene_name.to_string());
            io.set_int("Item ID", data.item_id as i32);

            Some("")
        }),
    );
}
