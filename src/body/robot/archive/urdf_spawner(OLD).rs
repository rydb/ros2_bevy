/*
 * Copyright (C) 2023 Open Source Robotics Foundation
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
*/

use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

use crate::body::robot::UrdfRoot;
use super::urdf_to_bevy::*;
use super::model_properties::*;
// use rmf_site_format::{
//     Anchor, Angle, Category, Link, MeshPrimitive, Pose, Rotation, UrdfRoot,
//     WorkcellCollisionMarker, WorkcellModel, WorkcellVisualMarker,
// };

use bevy::render::view::visibility::*;

use urdf_rs::JointType;

use bevy_rapier3d::na::geometry::Rotation as RapierRotation;
use bevy_rapier3d::prelude::*;
use bevy_rapier3d::rapier::math::Isometry;


pub fn handle_new_urdf_roots(mut commands: Commands, mut mesh_server: Res<Assets<Mesh>>, new_urdfs: Query<(Entity, &UrdfRoot)>) {
    let mut link_name_to_entity = HashMap::new();
    // Keep track of which links have a parent, add the ones that don't as a root child
    let mut root_links = HashSet::new();
    for (e, urdf) in new_urdfs.iter() {
        commands.entity(e).insert(RigidBody::KinematicVelocityBased);
        // Populate here
        for link in &urdf.links {
            // TODO*luca) link as child of anchor
            let link_entity = commands.spawn(
                (
                // A link without a physical model doesn't seem like it would spawn something, so i'm relegating spawning a physical "thing", to instances of visual links
                Visibility::Visible,
                ComputedVisibility::HIDDEN,
                // SpatialBundle {
                //     visibility: Visibility::Visible,
                //     computed: ComputedVisibility::HIDDEN,
                //     transform: Transform::IDENTITY,
                //     global_transform: GlobalTransform::IDENTITY
                // }, // Visible indetity changed to Inherieted identity?
                Link::from(link),
                RigidBody::KinematicVelocityBased,
                )
            )
            .id();
                //.insert(Category::Workcell)
            println!("Found link {:?} - {}", link_entity, link.name);
            link_name_to_entity.insert(link.name.clone(), link_entity);
            root_links.insert(link_entity);
            for visual in &link.visual {
                let model = BevyModel::from(visual);
                let cmd = commands.spawn(SpatialBundle::INHERITED_IDENTITY);
                let id = cmd.id();
                model.add_bevy_components(cmd);
                commands.entity(link_entity).add_child(id);
            }
            for collision in &link.collision {
                let model = BevyModel::from(collision);
                let cmd =
                    commands.spawn((SpatialBundle::INHERITED_IDENTITY, WorkcellCollisionMarker));
                let id = cmd.id();
                model.add_bevy_components(cmd);
                commands.entity(link_entity).add_child(id);
            }
        }
        for joint in &urdf.joints {
            if let Some(parent) = link_name_to_entity.get(&joint.parent.link) {
                if let Some(child) = link_name_to_entity.get(&joint.child.link) {
                    let trans = Vec3::from_array(joint.origin.xyz.map(|t| t as f32));
                    let rot = Vec3::from_array(joint.origin.rpy.map(|t| t as f32));
                    let rot = RapierRotation::from_euler_angles(rot[0], rot[1], rot[2]);
                    // TODO(luca) invert the above since it's in joint coordinates
                    let frame = Isometry::<f32>::from_parts(trans.into(), rot.into());
                    let joint_data = match joint.joint_type {
                        // A contious joint is the same thing as a revolute joint in the context of rapier(probably)
                        JointType::Revolute | JointType::Continuous => {
                            let axis = Vec3::from_array(joint.axis.xyz.map(|t| t as f32));
                            let joint = RevoluteJointBuilder::new(axis)
                                //.local_anchor2(trans)
                                .limits([joint.limit.lower as f32, joint.limit.upper as f32]);
                            ImpulseJoint::new(*parent, joint)
                        }
                        JointType::Prismatic => {
                            let axis = Vec3::from_array(joint.axis.xyz.map(|t| t as f32));
                            let joint = PrismaticJointBuilder::new(axis)
                                //.local_anchor2(trans)
                                .local_axis2(axis)
                                .limits([joint.limit.lower as f32, joint.limit.upper as f32]);
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
                        _ => {
                            todo!("Unimplemented joint type {:?}", joint.joint_type);
                        }
                    };
                    let trans = joint.origin.xyz.map(|t| t as f32);
                    let rot = Rotation::EulerExtrinsicXYZ(
                        joint.origin.rpy.map(|angle| Angle::Rad(angle as f32)),
                    );
                    //commands.entity(*child).insert(AnchorBundle::new(Anchor::Pose3D(Pose {trans, rot})));
                    commands.entity(*parent).add_child(*child);
                    root_links.remove(child);
                    println!(
                        "Adding joint between {:?} - {} and {:?} - {}",
                        *parent, &joint.parent.link, *child, &joint.child.link
                    );
                    commands.entity(*child).with_children(|children| {
                        children
                            .spawn(SpatialBundle::INHERITED_IDENTITY)
                            .insert(joint_data)
                            //.insert(Anchor::Pose3D(Pose { trans, rot }))
                            ;
                    });
                }
            }
        }
        for link in root_links.iter() {
            println!("Found root entity {:?}", link);
            commands.entity(e).add_child(*link);
        }
        commands.entity(e).remove::<UrdfRoot>();
    }
}

//pub fn spawn_models