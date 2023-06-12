//! Convert a urdf_rs bot to a bevy bot
//! 

use std::collections::{BTreeMap, HashSet};
use std::io;

use bevy::prelude::{Component, Deref, DerefMut};
use bevy::reflect::{TypeUuid};

use glam::Vec3;
use serde::{Deserialize, Serialize};
use urdf_rs::Robot;
use super::model_properties::{Rotation, Pose, Angle};


#[cfg_attr(
    feature = "bevy",
    derive(Component, Clone, Debug, Deref, DerefMut, TypeUuid)
)]
#[derive(Component, Clone, Debug, Deref, DerefMut, TypeUuid)]
#[uuid = "fe707f9e-c6f3-11ed-afa1-0242ac120002"]
pub struct UrdfRoot(pub Robot);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Geometry {
    //#[serde(flatten)]
    Primitive(MeshPrimitive),
    Mesh {
        filename: String,
        //(TODO)Serializing/Deserializing doesn't seem to work for this field. Should eventually figure out why."
        #[serde(default, skip)]
        scale: Option<Vec3>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Component))]
pub enum MeshPrimitive {
    Box { size: [f32; 3] },
    Cylinder { radius: f32, length: f32 },
    Capsule { radius: f32, length: f32 },
    Sphere { radius: f32 },
}

impl From<&urdf_rs::Geometry> for Geometry {
    fn from(geom: &urdf_rs::Geometry) -> Self {
        match geom {
            urdf_rs::Geometry::Box { size } => Geometry::Primitive(MeshPrimitive::Box {
                size: (**size).map(|f| f as f32),
            }),
            urdf_rs::Geometry::Cylinder { radius, length } => {
                Geometry::Primitive(MeshPrimitive::Cylinder {
                    radius: *radius as f32,
                    length: *length as f32,
                })
            }
            urdf_rs::Geometry::Capsule { radius, length } => {
                Geometry::Primitive(MeshPrimitive::Capsule {
                    radius: *radius as f32,
                    length: *length as f32,
                })
            }
            urdf_rs::Geometry::Sphere { radius } => Geometry::Primitive(MeshPrimitive::Sphere {
                radius: *radius as f32,
            }),
            urdf_rs::Geometry::Mesh { filename, scale } => {
                let scale = scale
                    .clone()
                    .and_then(|s| Some(Vec3::from_array(s.map(|v| v as f32))));
                Geometry::Mesh {
                    filename: filename.clone(),
                    scale,
                }
            }
        }
    }
}

pub struct BevyModel {
    pub name: String,
    pub geometry: Geometry,
    pub pose: Pose,
}



impl BevyModel {
    fn from_urdf_data(
        pose: &urdf_rs::Pose,
        name: &Option<String>,
        geometry: &urdf_rs::Geometry,
    ) -> Self {
        let trans = pose.xyz.map(|t| t as f32);
        let rot = Rotation::EulerExtrinsicXYZ(pose.rpy.map(|t| Angle::Rad(t as f32)));
        BevyModel {
            name: name.clone().unwrap_or_default(),
            geometry: geometry.into(),
            pose: Pose { trans, rot },
        }
    }
}