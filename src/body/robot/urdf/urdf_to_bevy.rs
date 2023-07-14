//! Convert a urdf_rs bot to a bevy bot
//! 

use std::collections::{BTreeMap, HashSet};
use std::io;

use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy::reflect::{TypeUuid, Reflect, };
use bevy::ecs::reflect::*;

use glam::Vec3;
use serde::{Deserialize, Serialize};
use urdf_rs::Robot;
use super::model_properties::*;
use super::recall::*;
use super::asset_source::*;
use bevy::reflect::TypePath;
pub trait RefTrait: Ord + Eq + Copy + Send + Sync + 'static {}

impl RefTrait for u32 {}

impl RefTrait for Entity {}

/// Helper structure to serialize / deserialize entities with parents
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Parented<P: RefTrait, T> {
    pub parent: P,
    #[serde(flatten)]
    pub bundle: T,
}

#[derive(Component, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MeshConstraint<T: RefTrait> {
    pub entity: T,
    pub element: MeshElement,
    pub relative_pose: Pose,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum MeshElement {
    Vertex(u32),
    // TODO(luca) edge and vertices
}

/// Attached to Model entities to keep track of constraints attached to them,
/// for change detection and hierarchy propagation
#[derive(Component, Deref, DerefMut, Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct ConstraintDependents(pub HashSet<Entity>);

#[derive(Component, Serialize, Deserialize, Debug, Default, Clone)]
pub struct WorkcellProperties {
    pub name: String,
}

#[derive(Component, Deref, DerefMut , Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct NameInWorkcell(pub String);

#[derive(Component, Deref, DerefMut, Serialize, Deserialize, Debug, Default, Clone)]
pub struct Mass(f32);

#[derive(Component, Serialize, Deserialize, Debug, Default, Clone)]
pub struct Inertia {}

#[derive(Bundle, Serialize, Deserialize, Debug, Default, Clone)]
pub struct Inertial {
    pub origin: Pose,
    pub mass: Mass,
    pub inertia: Inertia,
}

#[derive(Bundle, Serialize, Deserialize, Debug, Default, Clone)]
pub struct Link {
    pub name: NameInWorkcell,
    pub inertial: Inertial,
    #[serde(skip)]
    pub marker: LinkMarker,
}

#[derive(Component, Debug, Default, Clone)]
pub struct LinkMarker;

#[derive(Bundle, Serialize, Deserialize, Debug, Default, Clone)]
pub struct Joint {
    pub name: NameInWorkcell,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Geometry {
    //#[serde(flatten)]
    Primitive(MeshPrimitive),
    Mesh {
        filename: String,
        #[serde(skip)]
        scale: Option<Vec3>,
    },
}

#[derive(Component, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum MeshPrimitive {
    Box { size: [f32; 3] },
    Cylinder { radius: f32, length: f32 },
    Capsule { radius: f32, length: f32 },
    Sphere { radius: f32 },
}

impl MeshPrimitive {
    /// returns the bevy mesh equivilent of this enum variant..
    pub fn bevy_equiv(&self) -> Mesh{
        match &self {
            Self::Box { size } => Mesh::from(shape::Box {
                min_x: -size[0], max_x: size[0],
                min_y: -size[1], max_y: size[1],
                min_z: -size[2], max_z: size[2],
            }),
            Self::Cylinder { radius, length} => Mesh::from(shape::Cylinder{
                radius: *radius,
                height: *length,
                ..default()
            }),
            Self::Capsule { radius, length } => Mesh::from(shape::Capsule {
                radius: *radius,
                depth: *length, // this is probably not right... leaving this to not throw an error in case it is...
                ..default()
            }),
            Self::Sphere { radius} => Mesh::from(shape::Capsule {
                radius: *radius,
                depth: 0.0, // a capsule is a sphere if there is no mid section, and the icosphere doesnt work for Mesh::from....
                ..default()
            }),

        }
    }

    pub fn label(&self) -> String {
        match &self {
            MeshPrimitive::Box { .. } => "Box",
            MeshPrimitive::Cylinder { .. } => "Cylinder",
            MeshPrimitive::Capsule { .. } => "Capsule",
            MeshPrimitive::Sphere { .. } => "Sphere",
        }
        .to_string()
    }
}



#[derive(Component, Clone, Debug, Default, PartialEq)]
pub struct RecallMeshPrimitive {
    pub box_size: Option<[f32; 3]>,
    pub cylinder_radius: Option<f32>,
    pub cylinder_length: Option<f32>,
    pub capsule_radius: Option<f32>,
    pub capsule_length: Option<f32>,
    pub sphere_radius: Option<f32>,
}

impl Recall for RecallMeshPrimitive {
    type Source = MeshPrimitive;

    fn remember(&mut self, source: &MeshPrimitive) {
        match source {
            MeshPrimitive::Box { size } => {
                self.box_size = Some(*size);
            }
            MeshPrimitive::Cylinder { radius, length } => {
                self.cylinder_radius = Some(*radius);
                self.cylinder_length = Some(*length);
            }
            MeshPrimitive::Capsule { radius, length } => {
                self.capsule_radius = Some(*radius);
                self.capsule_length = Some(*length);
            }
            MeshPrimitive::Sphere { radius } => {
                self.sphere_radius = Some(*radius);
            }
        }
    }
}

impl RecallMeshPrimitive {
    pub fn assume_box(&self, current: &MeshPrimitive) -> MeshPrimitive {
        MeshPrimitive::Box {
            size: self.box_size.unwrap_or_default(),
        }
    }

    pub fn assume_cylinder(&self, current: &MeshPrimitive) -> MeshPrimitive {
        MeshPrimitive::Cylinder {
            radius: self.cylinder_radius.unwrap_or_default(),
            length: self.cylinder_length.unwrap_or_default(),
        }
    }

    pub fn assume_capsule(&self, current: &MeshPrimitive) -> MeshPrimitive {
        MeshPrimitive::Capsule {
            radius: self.capsule_radius.unwrap_or_default(),
            length: self.capsule_length.unwrap_or_default(),
        }
    }

    pub fn assume_sphere(&self, current: &MeshPrimitive) -> MeshPrimitive {
        MeshPrimitive::Sphere {
            radius: self.sphere_radius.unwrap_or_default(),
        }
    }
}

impl Default for Geometry {
    fn default() -> Self {
        Geometry::Primitive(MeshPrimitive::Box { size: [0.0; 3] })
    }
}

#[derive(Component, Debug, Default, Clone)]
pub struct WorkcellVisualMarker;

#[derive(Component, Debug, Default, Clone)]
pub struct WorkcellCollisionMarker;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
//#[reflect(Resource, Default)]
pub struct BevyModel {
    pub name: String,
    //#[reflect(ignore)]
    pub geometry: Geometry,
    //#[reflect(ignore)]
    pub pose: Pose,
}

impl BevyModel {
    pub fn add_bevy_components(&self, mut commands: EntityCommands) {
        match &self.geometry {
            Geometry::Primitive(primitive) => {
                
                
                println!("primtive model detected, spawning");
                commands.insert((
                    // PbrBundle {
                    //      mesh: mesh_server.add(primitive.bevy_equiv()),
                    //     ..default()
                    // },
                    primitive.clone(),
                    self.pose.clone(),
                    NameInWorkcell(self.name.clone()),
                ));
            }
            Geometry::Mesh { filename, scale } => {
                println!("mesh model detected, loading and spawning");
                println!("Setting pose of {:?} to {:?}", filename, self.pose);
                let scale = Scale(scale.unwrap_or_default());
                // TODO(luca) Make a bundle for workcell models to avoid manual insertion here
                commands.insert((
                    NameInWorkcell(self.name.clone()),
                    AssetSource::from(filename),
                    self.pose.clone(),
                    ConstraintDependents::default(),
                    scale,
                ));
            }
        }
    }
}


#[derive(Component, Clone, Debug, Deref, DerefMut, TypePath, TypeUuid)]
#[uuid = "fe707f9e-c6f3-11ed-afa1-0242ac120002"]
pub struct UrdfRoot(pub Robot);

// TODO(luca) feature gate urdf support
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

impl From<&urdf_rs::Link> for Link {
    fn from(link: &urdf_rs::Link) -> Self {
        Self {
            name: NameInWorkcell(link.name.clone()),
            inertial: Inertial {
                origin: Pose {
                    trans: link.inertial.origin.xyz.0.map(|v| v as f32),
                    rot: Rotation::EulerExtrinsicXYZ(
                        link.inertial.origin.rpy.map(|v| Angle::Rad(v as f32)),
                    ),
                },
                mass: Mass(link.inertial.mass.value as f32),
                inertia: Inertia::default(),
            },
            marker: LinkMarker,
        }
    }
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

impl From<&urdf_rs::Visual> for BevyModel {
    fn from(visual: &urdf_rs::Visual) -> Self {
        BevyModel::from_urdf_data(&visual.origin, &visual.name, &visual.geometry)
    }
}

impl From<&urdf_rs::Collision> for BevyModel {
    fn from(collision: &urdf_rs::Collision) -> Self {
        BevyModel::from_urdf_data(&collision.origin, &collision.name, &collision.geometry)
    }
}

