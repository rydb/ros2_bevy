use bevy::{prelude::*, pbr::wireframe::{Wireframe, WireframePlugin}};


use mesh_extras::{ramp::*, prelude::components::*};
use camera_extras::plugins::DefaultCameraPlugin;
use component_extras::components::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use editor_extras::plugins::{EditorPlugin, SelecterPlugin};
use ui_extras::systems::*;
//use editor_extras::plugins::EditorPlugin;
use mesh_extras::systems::*;
fn main() {
    App::new()
        .add_plugins(
            (
                DefaultPlugins,
                DefaultCameraPlugin,
                WorldInspectorPlugin::new(),
                SelecterPlugin,
                //WireframePlugin,
                //EditorPlugin, 
            )
        )
        .register_type::<MeshPull>()
        .add_systems(Startup, spawn_world/*, second_window_test*/)
        .add_systems(Update, (visualize_verticies, visualize_verticies_ui))
        .add_systems(Update, map_mesh_to_tugs)
        .add_systems(Update,  visualize_sidepanel_for::<Selected>)
        .add_systems(Update, select_specific_face)
        .run();
}

pub fn spawn_world (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(
        (
            PbrBundle {
                mesh: meshes.add(Ramp {
                    angle_of_incline: 45.0,
                    height: 1.0,
                    width: 1.0
                }.into()),
                material: materials.add(Color::GREEN.into()),
                ..default()
            },
            //Followed,
            Watched,
            Wireframe,
            
        ),
    );
}