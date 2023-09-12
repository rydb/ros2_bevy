use bevy::{prelude::*, pbr::wireframe::{Wireframe, WireframePlugin}};


use mesh_extras::ramp::*;
use camera_extras::plugins::DefaultCameraPlugin;
use component_extras::components::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use editor_extras::plugins::{EditorPlugin, SelecterPlugin};
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
        .add_systems(Startup, spawn_world/*, second_window_test*/)
        .add_systems(Update, (visualize_verticies, visualize_verticies_ui))
        .run();
}

pub fn spawn_world (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn(
        (
            PbrBundle {
                mesh: meshes.add(Ramp {
                    angle_of_incline: 45.0,
                    height: 1.0,
                    width: 1.0
                }.into()),
                ..default()
            },
            //Followed,
            Watched,
            Wireframe,
            
        ),
    );
}