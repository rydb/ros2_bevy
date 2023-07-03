use bevy::prelude::*;
use bevy::render::mesh::{self, PrimitiveTopology};

/// plugin for testing creating a custom mesh
pub struct CustomMeshTestPlugin;

impl Plugin for CustomMeshTestPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(Msaa::Sample4)
        .add_startup_system(spawn_custom_mesh)    
        ;
    }

}


// fn main() {
//     App::new()
//         .insert_resource(Msaa::Sample4)
//         .add_plugins(DefaultPlugins)
//         .add_startup_system(setup)
//         .run();
// }

fn spawn_custom_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    // verticies, eatch 3 vertex positions = 1 triangle. 
    // See https://bevy-cheatbook.github.io/features/coords.html
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            [0., 0., 0.], [1., 2., 1.], [2., 0., 0.],
            [0., 0., 0.], [-1., -2., -1.], [-2., 0., 0.]
            ],
        
        
    );

    // In this example, normals and UVs don't matter,
    // so we just use the same value for all of them
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0., 1., 0.]; 6]);
    //mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0., 0.]; 6]);

    // A triangle using vertices 0, 2, and 1.
    // Note: order matters. [0, 1, 2] will be flipped upside down, and you won't see it from behind!
    mesh.set_indices(Some(mesh::Indices::U32(vec![0, 2, 1])));
    commands.spawn(PbrBundle {
        mesh: meshes.add(mesh),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}