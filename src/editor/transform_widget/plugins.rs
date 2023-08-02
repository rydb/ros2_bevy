use bevy::prelude::*;

use super::systems::*;
use crate::DefaultRaycastingPlugin;
use bevy_mod_raycast::RaycastSystem;


/// plugin for managing transform widgets. Use this to spawn transform widgets to manipulate clicked models.
pub struct TransformWidgetPlugin;

impl Plugin for TransformWidgetPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(DefaultRaycastingPlugin::<SelectedForWidget>::default())
        .add_systems(
            First,add_gizmo_raycast.before(RaycastSystem::BuildRays::<SelectedForWidget>)
        )
        .add_systems(Update, (transform_widget_existence, transform_gizmo, add_gizmo_raycast))
        ;
    }
}