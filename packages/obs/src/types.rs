pub struct SceneCreated {
    pub scene_name: String,
    pub is_group: bool,
}

pub struct SceneRemoved {
    pub scene_name: String,
    pub is_group: bool,
}

pub struct SceneNameChanged {
    pub scene_name: String,
    pub old_scene_name: String,
}

pub struct SceneItemCreated {
    pub scene_name: String,
    pub source_name: String,
    pub item_id: u64,
    pub item_index: u32,
}

pub struct SceneItemRemoved {
    pub scene_name: String,
    pub input_name: String,
    pub item_id: u64,
}

pub struct SceneItemEnableChanged {
    pub scene_name: String,
    pub item_id: u64,
    pub enabled: bool,
}

pub struct SceneItemLockChanged {
    pub scene_name: String,
    pub item_id: u64,
    pub locked: bool,
}

pub struct SceneItemSelected {
    pub scene_name: String,
    pub item_id: u64,
}
