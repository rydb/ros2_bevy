mod body;
mod timers;
mod mesh;
mod editor;
mod urdf;
mod serialization;
mod worlds;

use bevy::prelude::*;

use worlds::plugins::*;
use bevy_flycam::prelude::*;
use bevy_mod_raycast::{
    DefaultRaycastingPlugin,
    RaycastSource,
};
use editor::plugins::EditorPlugin;
//use crate::body::cube::components::*;
use crate::serialization::plugins::SerializationPlugin;
fn main() {
    App::new()
        .add_plugins(
            (
                DefaultPlugins,//< --- bevy needs these in order to run
                RobotTestingWorld, // <-- World
                NoCameraPlayerPlugin, // <-- Camera
                EditorPlugin,
                SerializationPlugin, // <-- serialization
                

            )
        )
        //.add_systems(Update, serialize_world)
        .run();
}

//const NEW_SCENE_FILE_PATH: &str = "scenes/load_scene_example-new.scn.ron";


