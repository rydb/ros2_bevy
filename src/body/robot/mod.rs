use bevy::prelude::*;
use bevy_asset_loader::prelude::LoadingState;
use bevy_asset_loader::prelude::LoadingStateAppExt;

pub mod components;
mod systems;
pub mod custom_asset_loader_test;

//use crate::mesh::example::*;
use crate::body::robot::systems::*;
use crate::urdf::urdf_loader::*;
//use crate::urdf::urdf_spawner::*;
use crate::urdf::urdf_to_bevy::*;

use crate::timers::resources::*;

//use bevy_flycam::PlayerPlugin; // bevy 0.10
use bevy_rapier3d::prelude::*;
use bevy_obj::*;


//use super::robot::urdf::urdf_spawner::*;

use self::{custom_asset_loader_test::CustomAssetLoader, /*UrdfLoader*/};

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum AssetLoaderStates {
    #[default]
    AssetLoading,
    Next,
}

/// plugin for managing timers and ticking them in general. If there is a timer that needs to be managed, add its relevant system here
pub struct TimerManagerPlugin;

impl Plugin for TimerManagerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, tick_despawn_timer)
        ;
    }
}

/// plugin which contains all relevant custom asset loaders + initializes/adds all relevant
/// resources required to load them.
pub struct AssetLoadersPlugin;

impl Plugin for AssetLoadersPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_state::<AssetLoaderStates>()
        .add_plugins(
            (
                ObjPlugin, //adds support for .obj files 
                TimerManagerPlugin
            ) // manages timer functionality
        )
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
        .add_systems(OnEnter(AssetLoaderStates::Next), stage_robots_to_spawn_from_urdf)
        //.add_systems(Update, spawn_unspawned_robots)
        
        // Timers
        ;
    }
}

pub struct BasePlateWorld;

impl Plugin for BasePlateWorld {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(
            (
            AssetLoadersPlugin, // asset loaders
            //physics stuff -V
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default()
            )
        )
        
        .add_systems(Startup, (setup_physics, spawn_cube ))
        //.add_systems(Update, display_contacts)
        ;
    }
}

pub struct RobotTestPlugin;

///plugin for loading everything relevant to loading a robot
impl Plugin for RobotTestPlugin {
    fn build(&self, app: &mut App){
        app
        .add_plugins(BasePlateWorld) // World type.
        ;
    }
}

///plugin for testing individual systems with minimal overhead.
pub struct FeatureTestPlugin;
impl Plugin for FeatureTestPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(BasePlateWorld)
        //.add_plugin(CustomMeshTestPlugin)
        ;
    }
}


fn setup_physics(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,

) {
    println!("spawning ground");
    let base_plate_size = 100.0;
    /* Create the ground. */
    commands
        .spawn(
            PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane{
                size: base_plate_size * 2.0,
                subdivisions: 0,
            }
            )),
            material: materials.add(Color::WHITE.into()),
            transform: Transform::from_xyz(0.0, -5.0, 0.0),
            ..default()
        })
        .insert(Collider::cuboid(base_plate_size, 0.1, base_plate_size))
        .insert(Friction {coefficient: 1000.0, combine_rule: CoefficientCombineRule::Average})
        ;
}