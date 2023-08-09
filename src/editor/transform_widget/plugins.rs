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
        .add_systems(Update, widget_despawn_for_deselected)
        .add_systems(Update, (widget_spawn_for_selected,
            manage_x_tugs, manage_y_tugs, manage_z_tugs,
            manage_y_rings, manage_z_rings).after(widget_despawn_for_deselected)) // transform widget must exist while composing systems are running.
        ;
    }
}