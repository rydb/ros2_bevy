//! systems for a robot

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy::asset::*;
use crate::body::robot::resources::*;

pub const ASSET_FOLDER: &str = "assets/";
pub const URDF_FILE: &str = "diff_bot.xml";

///Spawn a simple differential drive based robot
pub fn spawn_robot(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,

    assset_server: Res<AssetServer>,

){
    // Meshes
    let body_mesh_handle: Handle<Mesh> = assset_server.load("meshes/diff_bot-BodyBase.obj");
    let left_wheel_mesh_handle: Handle<Mesh> = assset_server.load("meshes/left_wheel.obj");
    let right_wheel_mesh_handle: Handle<Mesh> = assset_server.load("meshes/right_wheel.obj");

    // Joints
    //let x = Vec::x_axis();
    //let joint = RevoluteJointBuilder::new(x);

    // main body
    let mut body_id = commands.spawn(

    (
            PbrBundle {
                mesh: body_mesh_handle,
                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                transform: Transform::from_xyz(0.0, 5.0, 0.0),
                ..default()
            },
            //physics properties
            RigidBody::Dynamic, // set this object to be a dynamic rigid body
            AsyncCollider(ComputedColliderShape::ConvexDecomposition
                (
                    default()
                )
            ),
        )
    ).id();
    let mut left_wheel_id = commands.spawn(

        (
                PbrBundle {
                    mesh: left_wheel_mesh_handle,
                    material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
                    transform: Transform::from_xyz(0.0, 50.0, 0.0),
                    ..default()
                },
                //physics properties
                RigidBody::Dynamic, // set this object to be a dynamic rigid body
                AsyncCollider(ComputedColliderShape::ConvexDecomposition
                    (
                        default()
                    )
                ),
            )
    ).id();
    let mut right_wheel_id = commands.spawn(

        (
                PbrBundle {
                    mesh: right_wheel_mesh_handle,
                    material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
                    transform: Transform::from_xyz(0.0, 50.0, 0.0),
                    ..default()
                },
                //physics properties
                RigidBody::Dynamic, // set this object to be a dynamic rigid body
                AsyncCollider(ComputedColliderShape::ConvexDecomposition
                    (
                        default()
                    )
                ),
            )
    ).id();

    // joints
    let z = Vec3::Z;
    
    
    let left_joint = RevoluteJointBuilder::new(z).local_anchor2(Vec3::new(1.07, 1.0, 0.5));
    let right_joint = RevoluteJointBuilder::new(z).local_anchor2(Vec3::new(-1.07, 1.0, 0.5));

    commands
        .entity(body_id)
        .with_children(
            |children| {
                children.spawn(ImpulseJoint::new(left_wheel_id, left_joint));
                children.spawn(ImpulseJoint::new(right_wheel_id, right_joint));

            }
        );
      
}

///print info relevant to a urdf
pub fn print_urdf(
    asset_server: Res<AssetServer>,
) {
    let mut urdf_folder_path = FileAssetIo::get_base_path()
    .join(ASSET_FOLDER.to_owned() + "urdfs/" + URDF_FILE)
    .to_str()
    .unwrap()
    .to_owned();

    let urdf_robot = urdf_rs::read_file(urdf_folder_path).unwrap();
    println!("{:?}", urdf_robot)
}