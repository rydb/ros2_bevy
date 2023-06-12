use std::default;

use bevy::prelude::*;

pub mod components;
mod systems;
pub mod resources;
pub mod custom_asset_loader_test;
mod urdf;

use crate::body::robot::systems::*;



use bevy::prelude::*;
use bevy_flycam::PlayerPlugin;
use bevy_rapier3d::prelude::*;
use bevy_obj::*;

use self::{resources::CountDownTimer, custom_asset_loader_test::CustomAssetLoader};

/// plugin which contains all relevant asset loaders
pub struct AssetLoadersPlugin;

impl Plugin for AssetLoadersPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_asset_loader(CustomAssetLoader)
        ;
    }
}


/// plugins for creating a base plate world. 
pub struct BasePlateWorld;

impl Plugin for BasePlateWorld {
    fn build(&self, app: &mut App) {
        app
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(PlayerPlugin)
        .add_plugin(AssetLoadersPlugin)


        
        //.add_plugin(CubePlugin)
        .add_plugin(ObjPlugin) // for loading obj meshes
        .add_startup_system(setup_physics)
        ;
    }
}

pub struct RobotTestPlugin;

///plugin for loading everything relevant to loading a robot
impl Plugin for RobotTestPlugin {
    fn build(&self, app: &mut App){
        app
        .add_plugin(BasePlateWorld) // World type.
        .add_startup_system(spawn_robots_from_urdf)
        ;
    }
}

///plugin for testing individual systems with minimal overhead.
pub struct FeatureTestPlugin;
impl Plugin for FeatureTestPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugin(BasePlateWorld)

        // resources
        .insert_resource(CountDownTimer::new(2))
        .add_startup_system(spawn_cube)
        .add_startup_system(spawn_robots_from_urdf)
        .add_system(move_robot_forward)
        .add_system(list_robots)
        ;
    }
}

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(100.0, 0.1, 100.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)));

}