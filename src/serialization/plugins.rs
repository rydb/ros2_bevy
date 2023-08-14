use bevy_inspector_egui::quick::WorldInspectorPlugin;

use std::{
    io,
    path::{Path, PathBuf},
};
use moonshine_save::{
    prelude::*,
    save::*,
};
    use bevy::prelude::*;

use super::components::*;
/// marks component as a valid candidate for serialization. 
// #[derive(Component)]
// pub struct Serializable;

const SAVE_PATH: &str = "cube.ron";

/// plugin that adds systems/plugins for serialization. 
/// `!!!THINGS THAT NEED TO BE SERIALIZED STILL MUST IMPLEMENT .register_type::<T>() IN ORDER TO BE USED!!!`
pub struct SerializationPlugin;

impl Plugin for SerializationPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(
            (SavePlugin, LoadPlugin)
        )
        .register_type::<ModelFlag>()
        .register_type::<Geometry>()
        .register_type::<MeshPrimitive>()
        //.add_systems(Update, save_into_file(SAVE_PATH).run_if(check_for_save_keypress))
        //.add_systems(Update, load_from_file(SAVE_PATH).run_if(check_for_load_keypress))
        //.add_systems(Update, spawn_unspawned_models)
        ;    
    }
}