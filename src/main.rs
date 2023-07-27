mod body;
mod timers;
mod mesh;
mod editor;

use std::f32::consts::PI;

use bevy::{prelude::*, reflect::TypePath, input::keyboard::KeyboardInput};
use bevy_rapier3d::prelude::{RigidBody, GravityScale};
use body::robot::{FeatureTestPlugin, RobotTestPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
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
                WorldInspectorPlugin::new(), // menu that displays active entities
                FeatureTestPlugin, // plugin which contains(mostly) everything program needs to run.
                NoCameraPlayerPlugin, // <-- Camera
                EditorPlugin,
                

            )
        )
        //.add_systems(Update, drive_wheels)
        .run();
}

// //drive wheels forward
// fn drive_wheels(
//     commands: Commands,
//     wheel_query: Query<(Entity,)
// ) {}