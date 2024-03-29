use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::timers::resources::DespawnTimer;
//use bevy_rapier3d::rapier::dynamics::JointAxis;
//use crate::serialization::components::{ModelFlag, SerializeType, Serializable};

// used to donote spawned model is a "part". Used to check
// for any models that the part is "bound" to.
#[derive(Component)]
pub struct Part;



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
            }
        }

    }
}



//drive wheels forward
// pub fn drive_wheels(
//     //commands: Commands,
//     mut wheel_query: Query<(Entity, &mut ImpulseJoint)>,
//     keys: Res<Input<KeyCode>>,

// ) {
//         // translation to be added after collecting all pressed key translation additions
//     // some of these are definatly wrong and will need tweaking...

//     // if reset rotation key is pressed, this should reset rotation to zero when set to true.
//     // let mut reset_rotation = false;

//     // // if this is enabled, model will be deselected during seelction checks for models.
//     // let mut deselect = false;
//     //vertical/horizontal rotations    
//     let speed_multiplier = 10.0;
    
//     let mut drive_velocity = 0.0;

//     // if keys.pressed(KeyCode::Space) {
//     //     direction_to_drive.translation += Vec3::new(0.0, 0.1, 0.0) * speed_multiplier
//     // }
//     // if keys.pressed(KeyCode::ShiftLeft) {
//     //     direction_to_drive.translation += Vec3::new(0.0, -0.1, 0.0) * speed_multiplier
//     // }
//     // if keys.pressed(KeyCode::Left) {
//     //     direction_to_drive.translation += Vec3::new(0.1, 0.0, 0.0) * speed_multiplier
//     // }
//     // if keys.pressed(KeyCode::Right) {
//     //     direction_to_drive.translation += Vec3::new(-0.1, 0.0, 0.0) * speed_multiplier
//     // }
//     if keys.pressed(KeyCode::Up) {
//         drive_velocity += -1.0 * speed_multiplier;
//         println!("driving twoards {:#?}", drive_velocity);

//     }
//     if keys.pressed(KeyCode::Down) {
//         drive_velocity += 1.0 * speed_multiplier;
//         println!("driving twoards {:#?}", drive_velocity);

//     }

//     for (_e, mut joint) in wheel_query.iter_mut() {
//         // commands.entity(e)
//         // .insert()
//         joint.data.set_motor_velocity(JointAxis::AngX, drive_velocity, speed_multiplier);
//     }
// }