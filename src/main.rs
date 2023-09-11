mod body;
mod timers;
//use mesh_extras;
use editor_extras::plugins::*;
mod urdf;
mod serialization;
mod worlds;
use camera_extras;

use bevy::prelude::*;

use worlds::plugins::*;
use bevy_flycam::prelude::*;
use bevy_mod_raycast::{
    DefaultRaycastingPlugin,
    RaycastSource,
};
//use editor_extras::plugins::EditorPlugin;
//use crate::body::cube::components::*;
use crate::serialization::plugins::SerializationPlugin;
fn main() {
    App::new()
        .add_plugins(
            (
                DefaultPlugins,//< --- bevy needs these in order to run
                TestingWorld, // <-- World
                EditorPlugin,
                SerializationPlugin, // <-- serialization
                

            )
        )
        //.add_systems(Update, serialize_world)
        .run();
}

//const NEW_SCENE_FILE_PATH: &str = "scenes/load_scene_example-new.scn.ron";


