use super::systems::*;
use bevy::prelude::*;
//use bevy_asset_loader::prelude::LoadingState;
//use bevy_asset_loader::prelude::LoadingStateAppExt;

//use crate::mesh::example::*;
use crate::urdf::urdf_loader::*;
use crate::urdf::urdf_spawner::*;
use crate::urdf::urdf_to_bevy::*;
use bevy_obj::*;


#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum UrdfLoaderState {
    #[default]
    AssetLoading,
    Next,
}

/// plugin which contians misc systems for displaying useful debug info for debugging robots.
pub struct RobotDebugPlugin;

impl Plugin for RobotDebugPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, display_contacts)
        ;
    }
}

/// plugin that manages everything related to spawning robots
pub struct RobotSpawnerPlugin;

impl Plugin for RobotSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(
            ObjPlugin,
        )
        .add_state::<UrdfLoaderState>()
        .init_resource::<SpawnableRobots>()
        .add_asset_loader(UrdfLoader) // Enables loading urdfs via `UrdfRoot` Supports .xml (TODO) Add .urdf support?
        .init_resource::<SpawnableRobots>()
        .add_asset::<UrdfRoot>()
        // .add_loading_state(
        //     LoadingState::new(UrdfLoaderState::AssetLoading).continue_to_state(UrdfLoaderState::Next)
        // )
        // .add_collection_to_loading_state::<_, SpawnableRobots>(UrdfLoaderState::AssetLoading)
        // .add_systems(OnEnter(UrdfLoaderState::Next), stage_robots_to_spawn_from_urdf)
        .add_systems(Startup, stage_robots_to_spawn_from_urdf)
        .add_systems(Update, spawn_unspawned_robots)
        
        ;

    }
}