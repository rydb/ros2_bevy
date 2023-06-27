use bevy::prelude::*;
//use crate::body::robot::components::ModelBundle;

use super::urdf_to_bevy::{UrdfRoot};
use super::urdf_loader::BevyRobot;
//use std::prelude::*;

use std::collections::{HashMap, HashSet};

use bevy_rapier3d::na::geometry::Rotation as RapierRotation;

use bevy_asset_loader::prelude::*;
use crate::body::robot::components::{ModelBundle, AssetSource};

use crate::Mesh;
use urdf_rs::Geometry::{Box, Cylinder, Capsule, Sphere, Mesh as UrdfMesh};
use urdf_rs::JointType;
use bevy_rapier3d::prelude::*;



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
                // keep a hash map of all links within the urdf for attaching joints later.
                let mut link_name_to_entity = HashMap::new();
                let mut root_links: HashSet<Entity> = HashSet::new();
                let spawned_robot = commands.entity(e).insert(
                    (
                        SpatialBundle::default()
                        // ComputedVisibility::HIDDEN,    
                        // GlobalTransform::from_xyz(0.0, 0.0, 0.0),
                        // //RigidBody::Dynamic
                    )
                    );
                    for link in &urdf.links {                        // for each part, spawn a sub part to be linked to the main robot later.
                        //println!("spawning link: {:#?}", link);
                        for visual_link in &link.visual {
                            //println!("spawning visual link, {:#?}", visual_link);
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
                            
                            link_name_to_entity.insert(link.name.clone(), model_entity);
                            root_links.insert(model_entity);


                        }
                        
                        // let part = commands.spawn(
                        //     (
                        //         ModelBundle::new(urdf, link.)
                        //     )
                        // )

                    }
                    // take joints, and form joint with 
                    for joint in &urdf.joints {
                        let parent_check = link_name_to_entity.get(&joint.parent.link);
                        let child_check = link_name_to_entity.get(&joint.child.link);
                        
                        let checks = parent_check.zip(child_check);
                        match checks {
                            Some(..) => {
                                println!("creating joint between models");
                                
                                let parent = parent_check.unwrap();
                                let child = child_check.unwrap();
                                
                                //let trans = Vec3::from_array(joint.origin.xyz.map(|t| t as f32));
                                let x = *joint.origin.xyz.get(0).unwrap() as f32;
                                let y = *joint.origin.xyz.get(1).unwrap() as f32;
                                let z = *joint.origin.xyz.get(2).unwrap() as f32;
                                
                                //let trans = Vec3::new(x, y, z);
                                let trans = Vec3::new((x.abs()/ x) *2.0, 1.0, 0.5);
                                println!("{:#?}",trans);
                                let rot = Vec3::from_array(joint.origin.rpy.map(|t| t as f32));
                                let rot = RapierRotation::from_euler_angles(rot[0], rot[1], rot[2]);
                                let joint_data = match joint.joint_type {
                                    JointType::Revolute  | JointType::Continuous => {
                                        let axis = Vec3::from_array(joint.axis.xyz.map(|t| t as f32));
                                        println!("axis is {:#?}", axis);
                                        let joint = RevoluteJointBuilder::new(axis)
                                            .local_anchor1(trans)
                                            .limits([joint.limit.lower as f32, joint.limit.upper as f32]);
                                            ;
                                        ImpulseJoint::new(*parent, joint)
                                    }
                                    JointType::Prismatic => {
                                        let axis = Vec3::from_array(joint.axis.xyz.map(|t| t as f32));
                                        let joint = PrismaticJointBuilder::new(axis)
                                            .local_anchor2(trans)
                                            .local_axis2(axis)
                                            .limits([joint.limit.lower as f32, joint.limit.upper as f32]);
                                        ImpulseJoint::new(*parent, joint)
                                    }
                                    JointType::Fixed => {
                                        let joint = FixedJointBuilder::new()
                                            .local_anchor1(trans)
                                            .local_anchor2(trans)
                                            .local_basis2(rot.into())
                                            ;
                                        ImpulseJoint::new(*parent, joint)
                                    }
                                    _ => {
                                        todo!("Unimplemented joint type {:?}", joint.joint_type);
                                    }
                                };
                                //let trans = joint.origin.xyz.map(|t| t as f32);
                                // let rot =
                                //     joint.origin.rpy.map(|angle| Angle::Rad(angle as f32)),
                                // );
                                //commands.entity(*child).insert(AnchorBundle::new(Anchor::Pose3D(Pose {trans, rot})));
                                commands.entity(*parent).add_child(*child);
                                root_links.remove(child);

                                println!(
                                    "Adding joint between {:?} - {} and {:?} - {}",
                                    *parent, &joint.parent.link, *child, &joint.child.link
                                );      
                                commands.entity(*child).with_children(|children| {
                                    children
                                        //.spawn(SpatialBundle::VISIBLE_IDENTITY)
                                        .spawn(joint_data)
                                        //.insert(Anchor::Pose3D(Pose { trans, rot }));
                                        ;
                                });
                                for link in root_links.iter() {
                                    println!("Found root entity {:?}", link);
                                    commands.entity(e).add_child(*link);
                                }
                            }

                            None => panic!("parent link evaluated to: {:#?}, child link evaluated to {:#?}.
                            both of these links must have a \"visual\" link defining their geometry in order for their
                            respective links to initialize. Make sure thats the case.", parent_check, child_check),
                        }
                        // if let Some(parent) = link_name_to_entity.get(&joint.parent.link) {
                        //     if let(Some)
                        // }
                        println!("attaching joints for: {:#?}", joint);
                    }
            },
            None => println!("urdf not loaded yet for current bot. load attempt aborted")
        }
        //println!("spawning, {:#}", urdf.name);
        

    }
}