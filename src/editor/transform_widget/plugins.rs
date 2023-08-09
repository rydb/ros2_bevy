use bevy::prelude::*;

use super::systems::*;
use crate::DefaultRaycastingPlugin;
use bevy_mod_raycast::RaycastSystem;
//use crate::editor::systems::SelectedForEdit;

/// plugin for managing transform widgets. Use this to spawn transform widgets to manipulate clicked models.
pub struct TransformWidgetPlugin;

//(todo) make a `Compose`, set which includes all composed systems, and have a `delete` set of systems, run only after compose
impl Plugin for TransformWidgetPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, widget_despawn_for_deselected)
        .add_systems(Update, (manage_tugs, manage_rings, widget_spawn_for_selected, transform_widget_behaviour)
            .before(widget_despawn_for_deselected)) 
            // COMPOSED SYSTEMS MUST RUN BEFORE DESPAWn BEHAVIOUR RUNS,
            // OTHERWISE, A CRASH FROM FAILING TO .insert(<thing>) INTO ENTITY WILL OCCUR
        ;
    }
}