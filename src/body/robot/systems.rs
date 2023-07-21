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
            Transform::from_xyz(0.0, 10.0, 0.0),
            materials.add(Color::PINK.into())
        )   
    );

}

/// create a green point at point(with a despawn timer!!) to show where contacts happenw within a model
#[allow(dead_code)]
pub fn display_contacts(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    //entity_mesh_querry: Query<(Entity, &Handle<Mesh>)>,
    rapier_context: Res<RapierContext>) {

    for contact in rapier_context.contact_pairs() {
        //println!("{:#?} and {:#?} collided with eachother", contact.collider1(), contact.collider2());
        for manifold in contact.manifolds() {
            for contact_point in manifold.points() {
                //contact_point.local_p1()
                
                //(todo) manually recoloring the triangles of the mesh is probably more efficient then spawning an entity every collision,
                // but that will need someone experienced in that or a bevy plugin...
                //https://github.com/bevyengine/bevy/blob/main/examples/animation/custom_skinned_mesh.rs

                // let e_check = entity_mesh_querry.get(contact.collider1());
                // match e_check {
                //     Ok(e) => {
                //         //println!("mesh is {:#?}", e.1);  
                //         if let Some(mut mesh) = meshes.get(e.1){
                //             //mesh.set_indices(indices)
                //             //println!("mesh topology is {:#?}", .());
                //         }  
                //     }
                //     Err(_) => {
                //         //println!("WARNING: query did not match for displaying contacts. Will want to make this more detailed later?")
                //     }
                // }
                
                //println!("first collider mehs is {:#?}", e.unwrap())
                let cube_size = 0.03 as f32;
                // make dots, make them into joints to connect to root mesh
                commands.spawn(
                    (
                        PbrBundle {
                            mesh: meshes.add(shape::Cube{size: cube_size}.into()),
                            material: materials.add(Color::rgb(0.1, 0.5, 0.3).into()),
                            transform: Transform::from_translation(contact_point.local_p1()),
                            ..default()
                        },
                        DespawnTimer::new(0.3 as f32),
                    )
                );
                // // commands.spawn(
                // //     (
                // //         ParticleBundle::new(
                // //             meshes.add(shape::Cube{size: cube_size}.into()),
                // //             materials.add(Color::rgb_u8(0, 191, 255).into()),
                // //             Transform::from_translation(contact_point.local_p2())
                // //         ),
                // //         DespawnTimer::new(0.3 as f32),
                // //     )
                // // );
                // let joint = FixedJointBuilder::new()
                //     .local_anchor1(contact_point.local_p1())
                //     //.local_anchor2(contact_point.local_p2())
                //     //.local_basis2(contact_point..into())
                //     ;
                // let joint_data = ImpulseJoint::new(contact.collider1(), joint);
                // commands.entity(contact.collider2()).with_children(|children| {
                //     children
                //         //.spawn(SpatialBundle::VISIBLE_IDENTITY)
                //         .spawn(joint_data)
                //         //.insert(Anchor::Pose3D(Pose { trans, rot }));
                //         ;
                // });
                
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
