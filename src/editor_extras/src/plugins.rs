use bevy::prelude::*;
use bevy_mod_raycast::RaycastSystem;
use bevy_mod_raycast::DefaultRaycastingPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_egui::EguiPlugin;
use bevy::pbr::wireframe::WireframePlugin;
use component_extras::components::*;
use crate::transform_widget::plugins::TransformWidgetPlugin;

use super::systems::*;
use crate::ui::*;
//use editor_extras::transform_widget::plugins::*;
/// plugin to click on stuff. Consolidates raycasts  into single plugin.
pub struct SelecterPlugin;

impl Plugin for SelecterPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(
            (
            DefaultRaycastingPlugin::<Selectable>::default(),
            EguiPlugin,
            TransformWidgetPlugin,
            WireframePlugin,
            )
        )
        .add_systems(
            First,update_raycast_with_cursor.before(RaycastSystem::BuildRays::<Selectable>)
        )
        .add_systems(Update, (inspector_ui, build_menu, hover_mesh_at_mouse))
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
        //.add_systems(Startup, spawn_debug_cam)
        .add_systems(Update, manage_selection_behaviour)
        ;
    }
}