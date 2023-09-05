use bevy::prelude::*;

use super::systems::*;


/// defualt camera for this project
pub struct DefaultCameraPlugin;

impl Plugin for DefaultCameraPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, spawn_debug_cam)
        .add_systems(Update, (follow_flagged, watch_flagged))
        ;
    }
}