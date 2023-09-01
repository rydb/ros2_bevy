use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::editor::components::MakeSelectableBundle;
use crate::editor::components::Selectable;
use crate::serialization::components::ModelFlag;
use crate::serialization::components::Serializable;
/// spawns a cube.
pub fn spawn_cube(
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
    //assset_server: Res<AssetServer>,

    //model_query: Query<Entity, With<BevyRobot>>,
) {
    // commands.spawn(ModelBundle::new(
    //     meshes.add(shape::Cube {size: 1.0}.into()),
    //     materials.add(Color::PINK.into()),
    //     Transform::from_xyz(0.0, 10.0, 20.0),
    //     SerializeType::SingleModel,
    // ));
    commands.spawn(
        (
        ModelFlag {
            geometry: shape::Cube {size: 1.0}.into(),
            //transform: Transform::from_xyz(0.0, 10.0, 20.0),
            material: Color::PINK.into()
        },
        Serializable,
        Transform::from_xyz(0.0, 10.0, 20.0),
        //SerializeType::SingleModel,   
    )
    );
    // );
    // commands.spawn(
    //     ModelBundle {
    //         pbr_bundle: PbrBundle {
    //             mesh: meshes.add(shape::Cube {size: 1.0}.into()),
    //             material: materials.add(Color::PINK.into()),
    //             transform: Transform::from_xyz(0.0, 10.0, 20.0),
    //             ..default()
    //         },
    //         ..Default::default()
    //     }

    // );
}

pub fn setup_physics(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,

) {
    println!("spawning ground");
    let base_plate_size = 100.0;
    /* Create the ground. */
    commands
        .spawn(
            PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane{
                size: base_plate_size * 2.0,
                subdivisions: 0,
            }
            )),
            material: materials.add(Color::WHITE.into()),
            transform: Transform::from_xyz(0.0, -5.0, 0.0),
            ..default()
        })
        .insert(Collider::cuboid(base_plate_size, 0.1, base_plate_size))
        .insert(Friction {coefficient: 1000.0, combine_rule: CoefficientCombineRule::Average})
        .insert(MakeSelectableBundle::default())
        .remove::<Selectable>() // we want the base-plate to be able to recieve selection events, but modifyable by widgets(yet)
        ;
}