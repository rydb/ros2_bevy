use bevy::prelude::*;
use crate::RigidBody;
use bevy_mod_raycast::RaycastSystem;
use crate::DefaultRaycastingPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use super::systems::*;

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(
            (
            DefaultRaycastingPlugin::<RigidBody>::default(),
            WorldInspectorPlugin::new(), // menu that displays active entities
            )
        )
        .add_systems(
            First,
            update_raycast_with_cursor.before(RaycastSystem::BuildRays::<RigidBody>),
        )
        .add_systems(Startup, (spawn_debug_cam))
        .add_systems(Update, (select_rigid_body, rigid_body_editor))
        ;
    }
}