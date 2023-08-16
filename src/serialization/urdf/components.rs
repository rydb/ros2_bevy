
// use bevy::prelude::*;
// use bevy::reflect::TypeUuid;
// // use urdf_rs::{
// //     Link,
// //     Inertial,
// //     Pose,
// // };


// use std::collections::HashSet;

// //use bevy::ecs::system::EntityCommands;
// use bevy::prelude::*;

// use glam::Vec3;
// use serde::{Deserialize, Serialize};
// use urdf_rs::Robot;
// //use super::recall::*;
// //use super::asset_source::*;
// use bevy::reflect::TypePath;

// #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
// pub enum Angle {
//     Deg(f32),
//     Rad(f32),
// }

// impl Angle {
//     pub fn radians(&self) -> f32 {
//         match self {
//             Angle::Deg(v) => v.to_radians(),
//             Angle::Rad(v) => *v,
//         }
//     }

//     pub fn degrees(&self) -> f32 {
//         match self {
//             Angle::Deg(v) => *v,
//             Angle::Rad(v) => v.to_degrees(),
//         }
//     }

//     pub fn match_variant(self, other: Angle) -> Self {
//         match other {
//             Angle::Deg(_) => Angle::Deg(self.degrees()),
//             Angle::Rad(_) => Angle::Rad(self.radians()),
//         }
//     }

//     pub fn is_radians(&self) -> bool {
//         matches!(self, Angle::Rad(_))
//     }

//     pub fn is_degrees(&self) -> bool {
//         matches!(self, Angle::Deg(_))
//     }
// }

// impl std::ops::Mul<f32> for Angle {
//     type Output = Angle;
//     fn mul(self, rhs: f32) -> Self::Output {
//         match self {
//             Self::Deg(v) => Self::Deg(rhs * v),
//             Self::Rad(v) => Self::Rad(rhs * v),
//         }
//     }
// }

// impl std::ops::Mul<Angle> for f32 {
//     type Output = Angle;
//     fn mul(self, rhs: Angle) -> Self::Output {
//         rhs * self
//     }
// }

// impl std::ops::Add for Angle {
//     type Output = Angle;
//     fn add(self, rhs: Self) -> Self::Output {
//         match self {
//             Self::Deg(v) => Self::Deg(v + rhs.degrees()),
//             Self::Rad(v) => Self::Rad(v + rhs.radians()),
//         }
//     }
// }

// impl std::ops::AddAssign for Angle {
//     fn add_assign(&mut self, rhs: Self) {
//         let result = *self + rhs;
//         *self = result;
//     }
// }

// impl std::ops::Sub for Angle {
//     type Output = Angle;
//     fn sub(self, rhs: Self) -> Self::Output {
//         match self {
//             Self::Deg(v) => Self::Deg(v - rhs.degrees()),
//             Self::Rad(v) => Self::Rad(v - rhs.radians()),
//         }
//     }
// }

// impl std::ops::SubAssign for Angle {
//     fn sub_assign(&mut self, rhs: Self) {
//         let result = *self - rhs;
//         *self = result;
//     }
// }

// #[derive(Debug, Clone, Copy, PartialEq)]
// pub enum Rotation {
//     Yaw(Angle),
//     EulerExtrinsicXYZ([Angle; 3]),
//     Quat([f32; 4]),
// }

// impl Rotation {
//     pub fn apply_yaw(&mut self, delta: Angle) {
//         match self {
//             Self::Yaw(yaw) => *yaw += delta,
//             Self::EulerExtrinsicXYZ([_, _, yaw]) => *yaw += delta,
//             Self::Quat(quat) => {
//                 let q = Quat::from_array(*quat);
//                 *quat = Quat::from_rotation_z(delta.radians())
//                     .mul_quat(q)
//                     .to_array();
//             }
//         }
//     }
// }

// //#[cfg(feature = "bevy")]
// impl Rotation {
//     pub fn as_yaw(&self) -> Self {
//         match self {
//             Self::Yaw(_) => self.clone(),
//             Self::EulerExtrinsicXYZ([_, _, yaw]) => Self::Yaw(*yaw),
//             Self::Quat(_) => Self::Yaw(Angle::Rad(self.as_bevy_quat().to_euler(EulerRot::ZYX).0)),
//         }
//     }

//     pub fn as_euler_extrinsic_xyz(&self) -> Self {
//         match self {
//             Self::Yaw(yaw) => Self::EulerExtrinsicXYZ([Angle::Deg(0.0), Angle::Deg(0.0), *yaw]),
//             Self::EulerExtrinsicXYZ(_) => self.clone(),
//             Self::Quat(_) => {
//                 let (z, y, x) = self.as_bevy_quat().to_euler(EulerRot::ZYX);
//                 Self::EulerExtrinsicXYZ([Angle::Rad(x), Angle::Rad(y), Angle::Rad(z)])
//             }
//         }
//     }

//     pub fn as_quat(&self) -> Self {
//         Self::Quat(self.as_bevy_quat().to_array())
//     }

//     pub fn as_bevy_quat(&self) -> Quat {
//         match self {
//             Self::Yaw(yaw) => Quat::from_rotation_z(yaw.radians()),
//             Self::EulerExtrinsicXYZ([x, y, z]) => {
//                 Quat::from_euler(EulerRot::ZYX, z.radians(), y.radians(), x.radians())
//             }
//             Self::Quat(quat) => Quat::from_array(*quat),
//         }
//     }

//     pub fn label(&self) -> &str {
//         match self {
//             Self::Yaw(_) => "Yaw",
//             Self::EulerExtrinsicXYZ(_) => "Euler Extrinsic XYZ",
//             Self::Quat(_) => "Quaternion",
//         }
//     }
// }

// impl Default for Rotation {
//     fn default() -> Self {
//         Rotation::Yaw(Angle::Deg(0.))
//     }
// }

// #[derive(Component, Debug, Clone, Copy, PartialEq)]
// pub struct Pose {
//     pub trans: [f32; 3],
//     pub rot: Rotation,
// }

// /// Helper structure to serialize / deserialize entities with parents
// // #[derive(Clone, Debug)]
// // pub struct Parented<P: RefTrait, T> {
// //     pub parent: P,
// //     pub bundle: T,
// // }

// // #[derive(Component, Debug, Clone, PartialEq)]
// // pub struct MeshConstraint<T: RefTrait> {
// //     pub entity: T,
// //     pub element: MeshElement,
// //     pub relative_pose: Pose,
// // }

// #[derive(Debug, Clone, PartialEq)]
// pub enum MeshElement {
//     Vertex(u32),
//     // TODO(luca) edge and vertices
// }

// /// Attached to Model entities to keep track of constraints attached to them,
// /// for change detection and hierarchy propagation
// #[derive(Component, Deref, DerefMut, Debug, Default, Clone, PartialEq)]
// pub struct ConstraintDependents(pub HashSet<Entity>);

// #[derive(Component, Debug, Default, Clone)]
// pub struct WorkcellProperties {
//     pub name: String,
// }

// #[derive(Component, Deref, DerefMut, Debug, Default, Clone, PartialEq)]
// pub struct NameInWorkcell(pub String);

// #[derive(Component, Deref, DerefMut, Debug, Default, Clone)]
// pub struct Mass(f32);

// #[derive(Component, Debug, Default, Clone)]
// pub struct Inertia {}

// #[derive(Bundle, Debug, Default, Clone)]
// pub struct Inertial {
//     pub origin: Pose,
//     pub mass: Mass,
//     pub inertia: Inertia,
// }

// #[derive(Bundle, Debug, Default, Clone)]
// pub struct Link {
//     pub name: NameInWorkcell,
//     pub inertial: Inertial,
//     pub marker: LinkMarker,
// }

// #[derive(Component, Debug, Default, Clone)]
// pub struct LinkMarker;

// #[derive(Bundle, Debug, Default, Clone)]
// pub struct Joint {
//     pub name: NameInWorkcell,
// }




// use crate::serialization::components::*;

// #[derive(Component, Clone, Debug, Deref, DerefMut, TypeUuid, Reflect)]
// #[uuid = "fe707f9e-c6f3-11ed-afa1-0242ac120002"]
// pub struct UrdfRoot(pub Robot);



// // TODO(luca) feature gate urdf support
// impl From<&urdf_rs::Geometry> for Geometry {
//     fn from(geom: &urdf_rs::Geometry) -> Self {
//         match geom {
//             urdf_rs::Geometry::Box { size } => Geometry::Primitive(MeshPrimitive::Box {
//                 size: (**size).map(|f| f as f32),
//             }),
//             urdf_rs::Geometry::Cylinder { radius, length } => {
//                 Geometry::Primitive(MeshPrimitive::Cylinder {
//                     radius: *radius as f32,
//                     length: *length as f32,
//                 })
//             }
//             urdf_rs::Geometry::Capsule { radius, length } => {
//                 Geometry::Primitive(MeshPrimitive::Capsule {
//                     radius: *radius as f32,
//                     length: *length as f32,
//                 })
//             }
//             urdf_rs::Geometry::Sphere { radius } => Geometry::Primitive(MeshPrimitive::Sphere {
//                 radius: *radius as f32,
//             }),
//             urdf_rs::Geometry::Mesh { filename, scale } => {
//                 let scale = scale
//                     .clone()
//                     .and_then(|s| Some(Vec3::from_array(s.map(|v| v as f32))));
//                 Geometry::Mesh {
//                     filename: filename.clone(),
//                     scale,
//                 }
//             }
//         }
//     }
// }

// impl From<&urdf_rs::Link> for Link {
//     fn from(link: &urdf_rs::Link) -> Self {
//         Self {
//             name: NameInWorkcell(link.name.clone()),
//             inertial: Inertial {
//                 origin: Pose {
//                     trans: link.inertial.origin.xyz.0.map(|v| v as f32),
//                     rot: Rotation::EulerExtrinsicXYZ(
//                         link.inertial.origin.rpy.map(|v| Angle::Rad(v as f32)),
//                     ),
//                 },
//                 mass: Mass(link.inertial.mass.value as f32),
//                 inertia: Inertia::default(),
//             },
//             marker: LinkMarker,
//         }
//     }
// }

// impl From<Robot> for ModelFlag {
//     fn from(value: Robot) -> Self {
        
//     }
// }

// #[derive(Debug, Default, Clone)]
// //#[reflect(Resource, Default)]
// pub struct BevyModel {
//     pub name: String,
//     pub geometry: Geometry,
//     pub pose: Pose,
// }

// impl ModelFlag {
//     fn from_urdf_data(
//         pose: &urdf_rs::Pose,
//         name: &Option<String>,
//         geometry: &urdf_rs::Geometry,
//     ) -> Self {
//         let trans = pose.xyz.map(|t| t as f32);
//         let rot = Rotation::EulerExtrinsicXYZ(pose.rpy.map(|t| Angle::Rad(t as f32)));
//         ModelFlag {
//             //name: name.clone().unwrap_or_default(),
//             geometry: geometry.into(),
//             transform: Transform::from { trans, rot },
//         }
//     }
// }

// impl From<&urdf_rs::Visual> for BevyModel {
//     fn from(visual: &urdf_rs::Visual) -> Self {
//         BevyModel::from_urdf_data(&visual.origin, &visual.name, &visual.geometry)
//     }
// }

// impl From<&urdf_rs::Collision> for BevyModel {
//     fn from(collision: &urdf_rs::Collision) -> Self {
//         BevyModel::from_urdf_data(&collision.origin, &collision.name, &collision.geometry)
//     }
// }