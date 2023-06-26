use bevy::prelude::*;
//use crate::body::robot::components::ModelBundle;

use super::urdf_to_bevy::{UrdfRoot};
use super::urdf_loader::BevyRobot;
//use std::prelude::*;

use bevy_asset_loader::prelude::*;
use crate::body::robot::components::{ModelBundle, AssetSource};

use crate::Mesh;
use urdf_rs::Geometry::{Box, Cylinder, Capsule, Sphere, Mesh as UrdfMesh};


// /// Find all robots without transforms, and construct a robot based on their urdf.
pub fn spawn_unspawned_robots(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    urdf_server: Res<Assets<UrdfRoot>>,
    asset_server: Res<AssetServer>,
    new_urdfs: Query<(Entity, &BevyRobot), Without<GlobalTransform>> 
) {

    //let mut robot = commands.spawn(Entity);
    for (e, unspawned_bot) in new_urdfs.iter() {
        let urdf_load_check = urdf_server.get(&unspawned_bot.urdf_file);
     
        match urdf_load_check {
            Some(urdf) => {
                println!("urdf is loaded, creating robot from urdf.");
                let spawned_robot = commands.entity(e).insert(
                    (
                        SpatialBundle::default()
                        // ComputedVisibility::HIDDEN,    
                        // GlobalTransform::from_xyz(0.0, 0.0, 0.0),
                        // //RigidBody::Dynamic
                    )
                    );
                    for link in &urdf.links {
                        // for each part, spawn a sub part to be linked to the main robot later.
                        //println!("spawning link: {:#?}", link);
                        for visual_link in &link.visual {
                            println!("spawning visual link, {:#?}", visual_link);
                            let model_mesh_handle = match &visual_link.geometry {
                                Box { size } => meshes.add(Mesh::from(shape::Box {
                                    min_x: -size[0] as f32, max_x: size[0] as f32,
                                    min_y: -size[1] as f32, max_y: size[1] as f32,
                                    min_z: -size[2]as f32, max_z: size[2] as f32,
                                })),
                                Cylinder { radius, length} => meshes.add(Mesh::from(shape::Cylinder{
                                    radius: *radius as f32,
                                    height: *length as f32,
                                    ..default()
                                })),
                                Capsule { radius, length } => meshes.add(Mesh::from(shape::Capsule {
                                    radius: *radius as f32,
                                    depth: *length as f32, // this is probably not right... leaving this to not throw an error in case it is...
                                    ..default()
                                })),
                                Sphere { radius} => meshes.add(Mesh::from(shape::Capsule {
                                    radius: *radius as f32,
                                    depth: 0.0, // a capsule is a sphere if there is no mid section, and the icosphere doesnt work for Mesh::from....
                                    ..default()
                                })),
                                UrdfMesh { filename, scale } => {
                                    // set filename to asset source, then set it back to string so path can be trimmed just for the filename + extension.
                                    // let asset_source= AssetSource::from(filename);
                                    // let cleaned_path = String::from(&asset_source);
                                    let split_paths: Vec<&str> = filename.split("/").collect();
                                    let model_file = *split_paths.last().unwrap();
                                    asset_server.load(unspawned_bot.models_dir_path.clone() + model_file)
                                },
                                //todo!("need to test this with model")//mesh_server.add(asset_server.load())
                    
                            };
                            let x = *visual_link.origin.xyz.get(0).unwrap() as f32;
                            let y = *visual_link.origin.xyz.get(1).unwrap() as f32;
                            let z = *visual_link.origin.xyz.get(2).unwrap() as f32;
                            let model = ModelBundle::new(
                                model_mesh_handle,
                                Transform::from_xyz(x, y, z)
                            );
                            let model_entity = commands.spawn(model).id();
                            commands.entity(e).add_child(model_entity);

                        }
                        // let part = commands.spawn(
                        //     (
                        //         ModelBundle::new(urdf, link.)
                        //     )
                        // )
                    }
            },
            None => println!("urdf not loaded yet for current bot. load attempt aborted")
        }
        //println!("spawning, {:#}", urdf.name);
        

    }
}