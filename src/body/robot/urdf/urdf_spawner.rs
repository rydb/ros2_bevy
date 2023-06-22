use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

use crate::body::robot::UrdfRoot;
use crate::body::robot::components::AssetSource;
use super::urdf_loader::SpawnedRobot;
use super::urdf_to_bevy::*;
use super::model_properties::*;
use crate::body::robot::components::{ModelBundle};
// use rmf_site_format::{
//     Anchor, Angle, Category, Link, MeshPrimitive, Pose, Rotation, UrdfRoot,
//     WorkcellCollisionMarker, WorkcellModel, WorkcellVisualMarker,
// };

//use bevy::render::view::visibility::*;

//use urdf_rs::JointType;

//use bevy_rapier3d::na::geometry::Rotation as RapierRotation;
use bevy_rapier3d::prelude::*;
//use bevy_rapier3d::rapier::math::Isometry;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum LoadState {
    #[default]
    Unloaded,
    Loaded,
}



// takes predefined UrdfRoot assets, and pairs them with  BevyRobot to be spawned.
pub fn spawn_urdf(
    mut commands: Commands,
    urdf_to_spawn: Res<SpawnedRobot>,
) {
    commands.spawn(
        (
            BevyRobot {
                urdf: urdf_to_spawn.urdf_handle,
                root_folder: "group_robot"
            }
        )
    )
}

/// finds all entities which are urdfs, and then spawns them.
pub fn spawn_robots_from_urdf(
    mut commands: Commands,
    mut mesh_server: Res<Assets<Mesh>>,
    new_urdfs: Query<(Entity, &BevyRobot), Without<Transform>> 
) {

    //let mut robot = commands.spawn(Entity);
    for (e, unspawned_bot) in new_urdfs.iter() {
        let urdf = &unspawned_bot.urdf;
        println!("spawning, {:#}", urdf.name);
        let spawned_robot = commands.entity(e).insert(
        (    
            Transform::from_xyz(0.0, 0.0, 0.0),
            RigidBody::Dynamic
        )
        );
        for link in &urdf.links {
            // for each part, spawn a sub part to be linked to the main robot later.
            println!("spawning link: {:#?}", link);
            for visual_link in &link.visual {
                ModelBundle::new()
            }
            // let part = commands.spawn(
            //     (
            //         ModelBundle::new(urdf, link.)
            //     )
            // )
        }

    }
}