use bevy::prelude::*;

use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins(
            (
                DefaultPlugins,
                WorldInspectorPlugin::new(),
            )
        )
        .add_systems(Startup, spawn_world)
        .run();
}

fn spawn_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn(
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        }
    );
    // commands.spawn(
    //     PbrBundle {
    //         mesh: meshse.a
    //     }
    // )
}