use bevy::prelude::*;

use super::systems::*;

/// plugin for managing transform widgets. Use this to spawn transform widgets to manipulate clicked models.
pub struct TransformWidgetPlugin;

impl Plugin for TransformWidgetPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, (transform_widget_existence, transform_gizmo))
        ;
    }
}