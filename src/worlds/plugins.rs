use bevy::prelude::*;

use crate::body::robot::plugins::RobotDebugPlugin;
use crate::body::robot::plugins::RobotSpawnerPlugin;
use crate::timers::plugins::TimerManagerPlugin;
//use crate::body::robot::TimerManagerPlugin;
//use crate::mesh::example::*;

use super::systems::*;

//use bevy_flycam::PlayerPlugin; // bevy 0.10
use bevy_rapier3d::prelude::*;
// plugin which contains all relevant custom asset loaders + initializes/adds all relevant
// resources required to load them.

/// plugin for managing timers and ticking them in general. If there is a timer that needs to be managed, add its relevant system here


pub struct BasePlateWorld;

impl Plugin for BasePlateWorld {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(
            (
            TimerManagerPlugin,
            RobotSpawnerPlugin, // asset loaders
            RobotDebugPlugin,
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
