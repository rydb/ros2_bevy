use bevy::prelude::*;

use super::systems::*;
use crate::DefaultRaycastingPlugin;
use bevy_mod_raycast::RaycastSystem;
//use crate::editor::systems::SelectedForEdit;

/// plugin for managing transform widgets. Use this to spawn transform widgets to manipulate clicked models.
pub struct TransformWidgetPlugin;

impl Plugin for TransformWidgetPlugin {
    fn build(&self, app: &mut App) {
        app
        //.add_plugins(DefaultRaycastingPlugin::<SelectedForWidget>::default())
        // .add_systems(
        //     First,add_gizmo_raycast.before(RaycastSystem::BuildRays::<SelectedForEdit>)
        // )
        .add_systems(Update, (transform_widget_existence, transform_gizmo))
        ;
    }
}