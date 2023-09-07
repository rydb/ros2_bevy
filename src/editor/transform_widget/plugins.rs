use bevy::prelude::*;

use super::{systems::*, gizmo_material::{GizmoMaterial, self}};
use crate::{editor::components::LastMouseInteraction};
//use crate::editor::systems::SelectedForEdit;
//use gizmo_material::GizmoMaterial;
/// plugin for managing transform widgets. Use this to spawn transform widgets to manipulate clicked models.
pub struct TransformWidgetPlugin;
use super::components::*;

//(todo) make a `Compose`, set which includes all composed systems, and have a `delete` set of systems, run only after compose
impl Plugin for TransformWidgetPlugin {
    fn build(&self, app: &mut App) {
//
        //let shader_path = "../../../assets/gizmo_material.wgsl";
        let mut shaders = app.world.get_resource_mut::<Assets<Shader>>().unwrap();
        shaders.set_untracked(
            gizmo_material::GIZMO_SHADER_HANDLE,
            Shader::from_wgsl(
                include_str!("../../../assets/gizmo_material.wgsl"),
                " ",//"../../../assets/gizmo_material.wgsl",
            ),
        );
        app

        .add_plugins(MaterialPlugin::<GizmoMaterial>::default())


        .register_type::<LastMouseInteraction>()
        .register_type::<Tug>()

        .add_systems(Update, widget_despawn_for_deselected)
        .add_systems(Update, (manage_tugs, manage_rings, widget_spawn_for_selected, transform_widget_behaviour)
            .before(widget_despawn_for_deselected)) 
            // COMPOSED SYSTEMS MUST RUN BEFORE DESPAWn BEHAVIOUR RUNS,
            // OTHERWISE, A CRASH FROM FAILING TO .insert(<thing>) INTO ENTITY WILL OCCUR
        ;
    }
}