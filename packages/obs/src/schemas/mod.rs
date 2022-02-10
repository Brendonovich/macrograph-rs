mod scene_items;
mod scenes;
mod streaming;

use self::{
    scene_items::create_scene_item_schemas, scenes::create_scene_schemas,
    streaming::create_streaming_schemas,
};

use macrograph_package_api::package::Package;

pub fn create_schemas(package: &mut Package) {
    create_scene_item_schemas(package);
    create_scene_schemas(package);
    create_streaming_schemas(package);
}
