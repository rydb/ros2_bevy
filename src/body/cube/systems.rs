
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::body::cube::resources::*;

///spawn a simple cube with a dynamic rigid body collider
pub fn spawn_cube(
    mut commands: Commands,
    mesh_server: ResMut<Assets<Mesh>>,
    assset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    cube_spawn_timer: ResMut<CubeTimer>,
){
    let body_mesh_handle: Handle<Mesh> = assset_server.load("meshes/diff_bot-BodyBase.obj");
    
    let cube_bundle = (
        PbrBundle {
            mesh: body_mesh_handle,
            //mesh: mesh_server.add(Mesh::from(shape::Cube {size: 1.0})), 
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 50.0, 0.0),
            ..default()
        },
        //physics properties
        RigidBody::Dynamic, // set this object to be a dynamic rigid body
        //AsyncCollider(ComputedColliderShape::TriMesh), //dynamically generates a collider from a mesh via teh AsyncCollider component
        AsyncCollider(ComputedColliderShape::ConvexDecomposition
            (
                default()
            )
        ),
        //Collider::cuboid(0.5, 0.5, 0.5),
        
        Restitution::coefficient(0.0),

    );

    //if(cube_spawn_timer.timer.finished()){
    commands.spawn(cube_bundle);
    //}


    
    //let wait_time = time::Duration::from_millis(10);
    //thread::sleep(wait_time);
    //commands.spawn(cube_bundle);




}

pub fn tick_cube_spawn_timer(mut cube_spawn_timer: ResMut<CubeTimer>, time: Res<Time>){
    cube_spawn_timer.timer.tick(time.delta());
}