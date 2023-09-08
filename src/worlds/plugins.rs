use bevy::prelude::*;

//use crate::body::robot::plugins::RobotDebugPlugin;
use crate::body::robot::plugins::RobotSpawnerPlugin;
use crate::timers::plugins::TimerManagerPlugin;
use crate::NoCameraPlayerPlugin;
use super::systems::*;
use bevy_rapier3d::prelude::*;

use crate::camera_extras::plugins::DefaultCameraPlugin;

/// world for testing misc things
pub struct TestingWorld;

impl Plugin for TestingWorld {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(
            (
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
            DefaultCameraPlugin,

            //NoCameraPlayerPlugin, // <-- Camera
            )
        )
        .add_systems( Startup,(spawn_base_plate, spawn_cube))
        ;
    }
}

/// Spawns a base plate + adds everyhting needed to test robot
pub struct RobotTestingWorld;

impl Plugin for RobotTestingWorld {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(
            (
            TimerManagerPlugin,
            RobotSpawnerPlugin, // asset loaders
            //RobotDebugPlugin,
            //physics stuff -V
            DefaultCameraPlugin,

            //NoCameraPlayerPlugin, // <-- Camera
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default()
            )
        )
        
        .add_systems(Startup, (spawn_base_plate, spawn_cube ))
        //.add_systems(Update, display_contacts)
        ;
    }
}
