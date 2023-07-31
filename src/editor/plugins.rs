use bevy::prelude::*;
use crate::RigidBody;
use bevy_mod_raycast::RaycastSystem;
use crate::DefaultRaycastingPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_egui::EguiPlugin;

use super::systems::*;

/// plugin to click on stuff. Consolidates raycasts  into single plugin.
pub struct SelecterPlugin;

impl Plugin for SelecterPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(
            (
            DefaultRaycastingPlugin::<RigidBody>::default(),
            EguiPlugin,
            )
        )
        .add_systems(
            First,update_raycast_with_cursor.before(RaycastSystem::BuildRays::<RigidBody>)
        )
        .add_systems(Update, inspector_ui)
        ;
    }
}

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(
            (
            SelecterPlugin,
            WorldInspectorPlugin::new(), // menu that displays active entities
            )
        )
        //.add_systems(RaycastSystem::BuildRays::<RigidBody>, update_raycast_with_cursor)
        .add_systems(Startup, (spawn_debug_cam))
        .add_systems(Update, (select_rigid_body, rigid_body_editor))
        ;
    }
}