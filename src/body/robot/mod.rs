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
use super::robot::urdf::urdf_loader::*;
use super::robot::urdf::urdf_spawner::*;
use super::robot::urdf::urdf_to_bevy::*;

use bevy::prelude::*;
use bevy_flycam::PlayerPlugin;
use bevy_rapier3d::prelude::*;
use bevy_obj::*;


//use super::robot::urdf::urdf_spawner::*;

use self::{resources::CountDownTimer, custom_asset_loader_test::CustomAssetLoader, urdf::urdf_loader::UrdfLoader};

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum AssetLoaderStates {
    #[default]
    AssetLoading,
    Next,
}
/// plugin which contains all relevant custom asset loaders + initializes/adds all relevant
/// resources required to load them.
pub struct AssetLoadersPlugin;

impl Plugin for AssetLoadersPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_state::<AssetLoaderStates>()
        .add_plugin(ObjPlugin) // .obj
        .add_asset_loader(CustomAssetLoader) // for loading CustomAsset Example
        
        // urdf loading stuff
        .add_asset_loader(UrdfLoader) // Enables loading urdfs via `UrdfRoot` Supports .xml (TODO) Add .urdf support?
        .init_resource::<SpawnableRobots>()
        .add_asset::<UrdfRoot>()
        //.add_system(stage_robots_to_initialize)
        .add_loading_state(
            LoadingState::new(AssetLoaderStates::AssetLoading).continue_to_state(AssetLoaderStates::Next)
        )
        .add_collection_to_loading_state::<_, SpawnableRobots>(AssetLoaderStates::AssetLoading)
        .add_system(stage_robots_to_spawn_from_urdf.in_schedule(OnEnter(AssetLoaderStates::Next)))
        .add_system(spawn_unspawned_robots)
        ;
    }
}

pub struct BasePlateWorld;

impl Plugin for BasePlateWorld {
    fn build(&self, app: &mut App) {
        app
        .add_plugin(AssetLoadersPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(PlayerPlugin)


        
        //.add_plugin(CubePlugin)
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
        //.add_startup_system(spawn_robots_from_urdf)
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
        
        //.add_system(spawn_robots_from_urdf)
        //.add_system(handle_new_urdf_roots)
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