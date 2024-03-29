use bevy::prelude::*;
//use crate::body::robot::components::ModelBundle;

use super::urdf_to_bevy::UrdfRoot;
use super::urdf_loader::BevyRobot;
//use std::prelude::*;
use crate::serialization::components::{ModelFlag, Serializable};
use std::collections::{HashMap, HashSet};
//use bevy_rapier3d::na::geometry::Rotation as RapierRotation;

//use crate::body::robot::components::{Wheel};
use super::components::*;

use urdf_rs::Geometry::Mesh as UrdfMesh;
use urdf_rs::JointType;
use bevy_rapier3d::prelude::*;


// /// Find all robots without transforms, and construct a robot based on their urdf.
pub fn spawn_unspawned_robots(
    mut commands: Commands,

    urdf_server: Res<Assets<UrdfRoot>>,
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
                commands.entity(e).insert((
                        SpatialBundle::default(),
                        Serializable,
                        //SerializeType::Urdf(unspawned_bot.urdf_path.clone()),
                )
                    ).insert(GlobalTransform::from_xyz(0.0, 10.0,0.0));
                    for link in &urdf.links {                        // for each part, spawn a sub part to be linked to the main robot later.
                        //println!("spawning link: {:#?}", link);
                        for visual_link in &link.visual {
                            //println!("spawning visual link, {:#?}", visual_link);

                            let model = match &visual_link.geometry {
                                UrdfMesh { filename, .. } => {
                                    // set filename to asset source, then set it back to string so path can be trimmed just for the filename + extension.
                                    // let asset_source= AssetSource::from(filename);
                                    // let cleaned_path = String::from(&asset_source);
                                    let split_paths: Vec<&str> = filename.split("/").collect();
                                    let model_file = *split_paths.last().unwrap();
                                    println!("loaded urdf model from: {:#?}", unspawned_bot.models_dir_path.clone() + model_file);

                                    let model_file_path = unspawned_bot.models_dir_path.clone() + model_file;
                                    //asset_server.load(unspawned_bot.models_dir_path.clone() + model_file)
                                    ModelFlag { 
                                        geometry: (&*model_file_path).into(),
                                        //transform: Transform::from_xyz(x, y, z),
                                        material: Color::PINK.into(),
                                        ..default()
                                    }
                                },
                                _ => ModelFlag {
                                    geometry: (&visual_link.geometry).into(),
                                    //transform: Transform::from_xyz(x, y, z),
                                    material: Color::PINK.into(),
                                    ..default()
                                },                    
                            };
                            let x = *visual_link.origin.xyz.get(0).unwrap() as f32;
                            let y = *visual_link.origin.xyz.get(1).unwrap() as f32;
                            let z = *visual_link.origin.xyz.get(2).unwrap() as f32;
                            let model_entity = commands.spawn(
                            (
                                model, 
                                Transform::from_xyz(x, y, z),
                            )
                            )
                            //make model not collide with it self for debuggign joints
                            .insert(CollisionGroups::new(Group::GROUP_1, Group::GROUP_10))
                            .insert(Damping{linear_damping: 0.0, angular_damping: 100.0})
                            //.insert(Transform::from_scale(0.0)) //for debug
                            .id();
                            commands.entity(e).add_child(model_entity);
                            
                            link_name_to_entity.insert(link.name.clone(), model_entity);
                            root_links.insert(model_entity);


                        }
                    }
                    // take joints, and form joint with 
                    for joint in &urdf.joints {
                        let parent_check = link_name_to_entity.get(&joint.parent.link);
                        let child_check = link_name_to_entity.get(&joint.child.link);
                        
                        let checks = parent_check.zip(child_check);
                        match checks {
                            Some(..) => {
                                //(TODO) .dae to .obj HOT FIX. BLENDER OVER SCALES MODEL BY 10X,
                                // THIS SHOULD BE REPLACED BY A PLUGIN TO GET MODEL CONVERSIONS VIA A BLENDER/FREECAD PLUGIN!!!
                                let blender_obj_overscale_correction = 10.0;
                                
                                println!("creating joint between models");
                                
                                let parent = parent_check.unwrap();
                                let child = child_check.unwrap();
                                
                                //let trans = Vec3::from_array(joint.origin.xyz.map(|t| t as f32));
                                let x = *joint.origin.xyz.get(0).unwrap() as f32;
                                let y = *joint.origin.xyz.get(1).unwrap() as f32;
                                let z = *joint.origin.xyz.get(2).unwrap() as f32;
                                
                                let trans = Vec3::new(x * blender_obj_overscale_correction, z * blender_obj_overscale_correction, y* blender_obj_overscale_correction);
                                //let trans = Vec3::new((x.abs()/ x) *0.75, 0.50, 0.40);
                                //println!("{:#?}",trans);
                                //let rot = Vec3::from_array(joint.origin.rpy.map(|t| t as f32));
                                //let rot = RapierRotation::from_euler_angles(rot[0], rot[1], rot[2]);
                                let joint_data = match joint.joint_type {
                                    JointType::Revolute => {
                                        let axis = Vec3::from_array(joint.axis.xyz.map(|t| t as f32));
                                        //println!("axis is {:#?}", axis);
                                        let joint = RevoluteJointBuilder::new(axis)
                                            //.local_anchor1(trans)
                                            .limits([(joint.limit.lower * 10.0) as f32  * blender_obj_overscale_correction, joint.limit.upper as f32 * blender_obj_overscale_correction])
                                            ;
                                        ImpulseJoint::new(*parent, joint)
                                    }
                                    JointType::Prismatic => {
                                        let axis = Vec3::from_array(joint.axis.xyz.map(|t| t as f32));
                                        let joint = PrismaticJointBuilder::new(axis)
                                            .local_anchor2(trans)
                                            .local_axis2(axis)
                                            .limits([joint.limit.lower as f32 * blender_obj_overscale_correction, joint.limit.upper as f32 * blender_obj_overscale_correction]);
                                        ImpulseJoint::new(*parent, joint)
                                    }
                                    JointType::Fixed => {
                                        let joint = FixedJointBuilder::new()
                                            .local_anchor1(trans)
                                            //.local_anchor2(trans)
                                            //.local_basis2(rot.into())
                                            ;
                                        ImpulseJoint::new(*parent, joint)
                                    }
                                    JointType::Continuous => {
                                        let axis = Vec3::from_array(joint.axis.xyz.map(|t| t as f32));
                                        //println!("axis is {:#?}", axis);
                                        let joint = RevoluteJointBuilder::new(axis)
                                            .local_anchor1(trans)
                                            //.limits([(joint.limit.lower * 10.0) as f32  * blender_obj_overscale_correction, joint.limit.upper as f32 * blender_obj_overscale_correction])
                                            //.motor_velocity(1.0, 0.1)
                                            ;
                                        commands.entity(*child)
                                        .insert(Wheel {});
//
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
                        //println!("attaching joints for: {:#?}", joint);
                    }
            },
            None => println!("urdf not loaded yet for current bot. load attempt aborted")
        }
        //println!("spawning, {:#}", urdf.name);
        

    }
}