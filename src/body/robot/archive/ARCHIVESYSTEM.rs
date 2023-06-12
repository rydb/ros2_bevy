//! systems for a robot

use bevy::prelude::*;
use bevy::utils::tracing::Instrument;
use bevy_rapier3d::prelude::*;
use bevy::asset::*;
use urdf_rs::Visual;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::convert::From;

use crate::body::robot::urdf;


//use crate::body::robot::resources::*;

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
    println!("Name: {:#?}", urdf_robot.name);
    //println!("Joints: {:#?}", urdf_robot.links);
    //println!("Links: {:#?}", urdf_robot.joints.);
    

    //for part in urdf_robot.links.iter() {
        //println!("First_link: {:#?}", part)
    //}
    //for part in urdf_robot{

    //}
    //println!("First_link: {:#?}", urdf_robot.joints);
    
    //println!("Materials: {:#?}", urdf_robot.materials);
    
    //println!("Unsorted info: {:#?}", )
    //println!("Unsorted info: {:#?}", urdf_robot)


}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Component))]
pub enum AssetSource {
    Local(String),
    Remote(String),
    Search(String),
    Bundled(String),
    Package(String),
}

impl AssetSource {
    pub fn label(&self) -> &str {
        match self {
            Self::Local(_) => "Local",
            Self::Remote(_) => "Remote",
            Self::Search(_) => "Search",
            Self::Bundled(_) => "Bundled",
            Self::Package(_) => "Package",
        }
    }
}

impl Default for AssetSource {
    fn default() -> Self {
        AssetSource::Local(String::new()).into()
    }
}

// Utility functions to add / strip prefixes for using AssetSource in AssetIo objects
impl From<&Path> for AssetSource {
    fn from(path: &Path) -> Self {
        if let Some(path) = path.to_str().and_then(|p| Some(String::from(p))) {
            AssetSource::from(&path)
        } else {
            AssetSource::default()
        }
    }
}

// Utility functions to add / strip prefixes for using AssetSource in AssetIo objects
impl From<&String> for AssetSource {
    fn from(path: &String) -> Self {
        // TODO(luca) pattern matching here would make sure unimplemented variants are a compile error
        if let Some(path) = path.strip_prefix("rmf-server://").map(|p| p.to_string()) {
            return AssetSource::Remote(path);
        } else if let Some(path) = path.strip_prefix("file://").map(|p| p.to_string()) {
            return AssetSource::Local(path);
        } else if let Some(path) = path.strip_prefix("search://").map(|p| p.to_string()) {
            return AssetSource::Search(path);
        } else if let Some(path) = path.strip_prefix("bundled://").map(|p| p.to_string()) {
            return AssetSource::Bundled(path);
        } else if let Some(path) = path.strip_prefix("package://").map(|p| p.to_string()) {
            return AssetSource::Package(path);
        }
        AssetSource::default()
    }
}

impl From<&AssetSource> for String {
    fn from(asset_source: &AssetSource) -> String {
        match asset_source {
            AssetSource::Remote(uri) => String::from("rmf-server://") + uri,
            AssetSource::Local(filename) => String::from("file://") + filename,
            AssetSource::Search(name) => String::from("search://") + name,
            AssetSource::Bundled(name) => String::from("bundled://") + name,
            AssetSource::Package(path) => /*String::from("package://") + */ path.to_owned(), //package part of papckage is not needed for now..
        }
    }
}

///spawn a robot based on a urdf file store in assets/urdfs/
pub fn spawn_robot_from_urdf(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,

    assset_server: Res<AssetServer>,
) {

    let mut urdf_folder_path = FileAssetIo::get_base_path()
    .join(ASSET_FOLDER.to_owned() + "urdfs/" + URDF_FILE)
    .to_str()
    .unwrap()
    .to_owned();

    let urdf_robot = urdf_rs::read_file(urdf_folder_path).unwrap();

    //get all meshes
    //get all links
    //get all get all cordinates of parts
    //create bundles
    //link them to eachother

    // basically, compose an object from all of its properties:
    // 1. spawn an object from its mesh
    for links in urdf_robot.links {
        for visual in links.visual {
            //println!("{:#?} ", visual)
            match visual.geometry {
                urdf_rs::Geometry::Box { size } => println!("box detected!"),
                urdf_rs::Geometry::Cylinder { radius, length } => println!("Cylinder detected!"),
                urdf_rs::Geometry::Capsule { radius, length } => println!("Mesh detected!"),
                urdf_rs::Geometry::Sphere { radius } => println!("Sphere detected"),
                urdf_rs::Geometry::Mesh { filename, scale } => {
                    let scale = scale
                        .clone()
                        .and_then(|s| Some(Vec3::from_array(
                            s.map(|v| v as f32)
                        )));
                    //println!("package type + file name of mesh is: {:#}", filename);

                    let urdf_file_path = String::from(&AssetSource::from(&filename));

                    //get urdf path urdf file
                    println!("urdf file path is: {:#?}", urdf_file_path);


                }//println!("Mesh detected!"),
            }
        }
    }
    //for meshes in urdf_robot.links[0].visual[0].geometry
}