mod body;
mod timers;
mod mesh;
mod editor;
mod urdf;
mod serialization;

use bevy::{prelude::*, reflect::TypePath, input::keyboard::KeyboardInput, tasks::IoTaskPool};
use std::{fs::File, io::Write};

use bevy_rapier3d::prelude::{RigidBody, GravityScale, ImpulseJoint};
use body::robot::{FeatureTestPlugin, RobotTestPlugin};
use bevy_flycam::prelude::*;
use bevy_mod_raycast::{
    print_intersections, DefaultPluginState, DefaultRaycastingPlugin, RaycastMesh, RaycastMethod,
    RaycastSource, RaycastSystem,
};
use editor::plugins::EditorPlugin;
//use crate::body::cube::components::*;

fn main() {
    App::new()
        .add_plugins(
            (
                DefaultPlugins,//< --- bevy needs these in order to run
                FeatureTestPlugin, // plugin which contains(mostly) everything program needs to run.
                NoCameraPlayerPlugin, // <-- Camera
                EditorPlugin,
                

            )
        )
        //.add_systems(Update, serialize_world)
        .run();
}

const NEW_SCENE_FILE_PATH: &str = "scenes/load_scene_example-new.scn.ron";


