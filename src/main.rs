mod body;


use bevy::prelude::*;
use bevy_flycam::PlayerPlugin;
use bevy_rapier3d::prelude::*;
use bevy_obj::*;
use body::robot::{FeatureTestPlugin, RobotTestPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
//use crate::body::cube::components::*;
use crate::body::robot::components::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins) //< --- bevy needs these in order to run
        
        .add_plugin(FeatureTestPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        //.add_plugin(FeatureTestPlugin)
        // MAIN PLUGIN!!! DETERMINES SCENE TYPE

        .run();
}

