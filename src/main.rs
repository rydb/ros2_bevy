mod body;

use bevy::prelude::*;
use bevy_flycam::PlayerPlugin;
use bevy_rapier3d::prelude::*;
use bevy_obj::*;
use body::robot::{FeatureTestPlugin, RobotTestPlugin, self, custom_asset_loader_test::CustomAsset};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
//use crate::body::cube::components::*;
use crate::body::robot::components::*;
use crate::robot::custom_asset_loader_test::*;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins) //< --- bevy needs these in order to run
        .add_plugin(WorldInspectorPlugin::new())
        // .init_resource::<body::robot::custom_asset_loader_test::State>()
        // .add_asset::<CustomAsset>()
        // .add_asset_loader(CustomAssetLoader)
        // .add_startup_system(setup)
        // .add_system(print_on_load)
        //::<robot::custom_asset_loader_test::CustomAssetLoader>()
        .add_plugin(FeatureTestPlugin)
        // MAIN PLUGIN!!! DETERMINES SCENE TYPE

        .run();
}

