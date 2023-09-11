use bevy::prelude::*;


use mesh_extras::ramp::*;
use camera_extras::plugins::DefaultCameraPlugin;
use component_extras::components::*;

fn main() {
    App::new()
        .add_plugins(
            (
                DefaultPlugins,
                DefaultCameraPlugin,
            )
        )
        .add_systems(Startup, spawn_world)
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
            
        ),
    );
}