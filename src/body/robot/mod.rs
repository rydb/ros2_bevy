use std::default;

use bevy::prelude::*;
use bevy_asset_loader::prelude::LoadingState;
use bevy_asset_loader::prelude::LoadingStateAppExt;

pub mod components;
mod systems;
pub mod resources;
pub mod custom_asset_loader_test;
pub mod urdf;

use crate::body::robot::systems::*;
use crate::robot::urdf::urdf_loader::*;
use crate::robot::urdf::urdf_to_bevy::*;

use bevy::prelude::*;
use bevy_flycam::PlayerPlugin;
use bevy_rapier3d::prelude::*;
use bevy_obj::*;

use self::{resources::CountDownTimer, custom_asset_loader_test::CustomAssetLoader, urdf::urdf_loader::UrdfLoader};


#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum MyStates {
    #[default]
    AssetLoading,
    Next,
}

/// plugin which contains all relevant asset loaders
pub struct AssetLoadersPlugin;

impl Plugin for AssetLoadersPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_asset_loader(CustomAssetLoader)
        .add_asset_loader(UrdfLoader)
        ;
    }
}


/// plugins for creating a base plate world. 
pub struct BasePlateWorld;

impl Plugin for BasePlateWorld {
    fn build(&self, app: &mut App) {
        app
        .add_state::<MyStates>()
        .add_asset::<UrdfRoot>()
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(PlayerPlugin)
        .add_plugin(AssetLoadersPlugin)
        .add_loading_state(
            LoadingState::new(MyStates::AssetLoading).continue_to_state(MyStates::Next)
        )
        .add_collection_to_loading_state::<_, SpawnedRobot>(MyStates::AssetLoading)
        .add_system(load_diff_bot.in_schedule(OnEnter(MyStates::Next)))

        
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
        .init_resource::<SpawnedRobot>()
        .insert_resource(CountDownTimer::new(2))

        // Assets
        //.add_collection_to_loading_state::<UrdfRoot>()
        //.add_startup_system(spawn_cube)
        //.add_startup_system(spawn_robots_from_urdf)
        //.add_system(move_robot_forward)
        //.add_system(list_robots)

        //.add_startup_system(setup_diff_bot)
        //.add_system(load_diff_bot) //<-- this needs to be run after setting up loading urdfs. or this will error from not finding the urdf
        ;
    }
}

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(100.0, 0.1, 100.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)));

}