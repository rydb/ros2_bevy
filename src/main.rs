mod body;
mod timers;
mod mesh;
//mod cameras;

use bevy::prelude::*;
use body::robot::{FeatureTestPlugin, RobotTestPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_flycam::prelude::*;
//use crate::body::cube::components::*;

fn main() {
    App::new()
        .add_plugins(
            (
                DefaultPlugins,//< --- bevy needs these in order to run
                WorldInspectorPlugin::new(), // menu that displays active entities
                FeatureTestPlugin, // plugin which contains(mostly) everything program needs to run.
                 // <-- Camera

            )
        )
        .add_plugins(PlayerPlugin) 
        .run();
}

