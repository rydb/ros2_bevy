mod body;
mod timers;
mod mesh;

use bevy::prelude::*;
use body::robot::{FeatureTestPlugin, RobotTestPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
//use crate::body::cube::components::*;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins) //< --- bevy needs these in order to run
        .add_plugin(WorldInspectorPlugin::new())

        // .add_asset::<CustomAsset>()
        // .add_asset_loader(CustomAssetLoader)
        // .add_startup_system(setup)
        // .add_system(print_on_load)
        //::<robot::custom_asset_loader_test::CustomAssetLoader>()
        .add_plugin(FeatureTestPlugin)
        // MAIN PLUGIN!!! DETERMINES SCENE TYPE

        .run();
}

