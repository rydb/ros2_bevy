mod systems;
mod components;

use bevy::prelude::*;


use bevy_fly_camera::FlyCameraPlugin;
use bevy_flycam::PlayerPlugin;
use bevy_flycam::MovementSettings;
use bevy_flycam::KeyBindings;

use systems::*;
use components::*;

/// Camera plugin for default camera with default functionality
pub struct DefaultCamera;

impl Plugin for DefaultCamera {
    fn build(&self, app: &mut App) {
        app 
        .add_plugin(FlyCameraPlugin)
        .add_startup_system(spawn_fly_camera)
        ;
        //Startup Systems
        //.add_startup_system(spawn_camera)
        //;
    }
}