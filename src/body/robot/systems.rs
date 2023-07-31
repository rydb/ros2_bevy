use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::timers::resources::DespawnTimer;

use crate::body::robot::components::*;

// used to donote spawned model is a "part". Used to check
// for any models that the part is "bound" to.
#[derive(Component)]
pub struct Part;

///TODO
/// 1. ADD ROBOT SPAWNING
/// 2. ADD ROBOT MOVING


pub fn spawn_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    //assset_server: Res<AssetServer>,

    //model_query: Query<Entity, With<BevyRobot>>,
) {
    commands.spawn(
        ModelBundle::new(
            meshes.add(Mesh::from(shape::Cube {size: 1.0})),
            Transform::from_xyz(0.0, 10.0, 20.0),
            materials.add(Color::PINK.into())
        )   
    );

}

/// checks for collisions, and briefly spawns cubes to showcase 
#[allow(dead_code)]
pub fn display_contacts(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    transforms: Query<&GlobalTransform>,
    rapier_context: Res<RapierContext>)
     {

    for contact in rapier_context.contact_pairs() {
        //println!("{:#?} and {:#?} collided with eachother", contact.collider1(), contact.collider2());
        for manifold in contact.manifolds() {
            //println!("contact points are: {:#?}", manifold.points());
            for contact_point in manifold.points() {
                let collider1_transform = transforms.get(contact.collider2()).unwrap();
                let local_contact_point = contact_point.local_p2();
                let collision_point = collider1_transform.transform_point(local_contact_point);
                let cube_size = 0.1 as f32;

                println!("collider global transform is: {:#?}", collider1_transform);
                println!("contact point is {:#?}", local_contact_point);
                println!("collision happened at: {:#?}", collision_point);
                commands.spawn(
                    (
                        PbrBundle {
                            mesh: meshes.add(shape::Cube{size: cube_size}.into()),
                            material: materials.add(Color::RED.into()),
                            transform: Transform::from_translation(collider1_transform.translation()),
                            ..default()
                        },
                        DespawnTimer::new(0.3 as f32),
                    )
                ); 
                commands.spawn(
                    (
                        PbrBundle {
                            mesh: meshes.add(shape::Cube{size: cube_size}.into()),
                            material: materials.add(Color::PURPLE.into()),
                            transform: Transform::from_translation(collision_point),
                            ..default()
                        },
                        DespawnTimer::new(0.3 as f32),
                    )
                );   
                commands.spawn(
                    (
                        PbrBundle {
                            mesh: meshes.add(shape::Cube{size: cube_size}.into()),
                            material: materials.add(Color::GREEN.into()),
                            transform: Transform::from_translation(local_contact_point),
                            ..default()
                        },
                        DespawnTimer::new(0.3 as f32),
                    )
                );        
            }
        }

    }
}


// moves all robots forward(knowing the total forces being exerted on the collider would be helpful? Mabye for establishing some kind of formula?)
pub fn move_robot_forward(
    //mut model_query: Query<&mut ExternalForce>,
    //mut timer_query: ResMut<CountDownTimer>,
    //time: Res<Time>,
    
) {
    //todo!("not implemented")
}
