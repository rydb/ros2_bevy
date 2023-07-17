mod systems;
mod components;

use bevy::prelude::*;


use bevy_flycam::*;
use systems::*;
use components::*;

/// Camera plugin for default camera with default functionality
pub struct DefaultCamera;

impl Plugin for DefaultCamera {
    fn build(&self, app: &mut App) {
        app 
        .add_plugin(PlayerPlugin)
        .insert_resource(MovementSettings {
            sensitivity: 0.00015, // default: 0.00012
            speed: 12.0, // default: 12.0
        })
        .insert_resource(KeyBindings {
            move_ascend: KeyCode::E,
            move_descend: KeyCode::Q,
            ..Default::default()
        })
        ;
        //Startup Systems
        //.add_startup_system(spawn_camera)
        //;
    }
}

