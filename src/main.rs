mod body;
mod timers;
mod mesh;
//mod cameras;

use bevy::prelude::*;
use body::robot::{FeatureTestPlugin, RobotTestPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_flycam::prelude::*;
//use crate::body::cube::components::*;

#[bevy_main]
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
         // bevy 0.10

        // .add_asset::<CustomAsset>()
        // .add_asset_loader(CustomAssetLoader)
        // .add_startup_system(setup)
        // .add_system(print_on_load)
        //::<robot::custom_asset_loader_test::CustomAssetLoader>()
        // MAIN PLUGIN!!! DETERMINES SCENE TYPE

        .run();
}

