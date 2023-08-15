use bevy::prelude::*;
use bevy::render::mesh::shape::*;
use bevy::reflect::TypeUuid;
use urdf_rs::Robot;

/// component which flags entity as a model for spawning purposes. !!!TREAT THIS AS READ ONLY!!!
#[derive(Component, Reflect, Clone)]
//#[reflect(from_reflect = false)]
#[reflect(Component)]
pub struct ModelFlag {
    pub geometry: Geometry,
    pub material: StandardMaterial,
    pub transform: Transform, 
}

impl Default for ModelFlag {
    fn default() -> Self {
        Self {
            geometry: Default::default(),
            material: Default::default(),
            transform: Default::default()
        }
    }
}

/// geometry type. Should only be set once and left unedited. 
#[derive(Component, Reflect, Clone)]
//#[reflect(from_reflect = false)]
#[reflect(Component)]
pub enum Geometry{
    Primitive(MeshPrimitive),
    Mesh {
        filename: String,
        scale: Vec3,
    },
}

/// Reflect, and Serialization both require a default implementation of structs. The default GeometryFlag resorts to an "fallback" mesh to
/// represent failed load attempts. (TODO): add a system that picks up error meshes, and displays them somewhere.
impl Default for Geometry {
    fn default() -> Self {
        Self::Mesh {
            filename: "fallback.gltf".to_string(),
            scale: Vec3::new(0.0,0.0,0.0),
        }        
    }
}

#[derive(Debug, Clone, PartialEq, Reflect, Copy)]
#[derive(Component)]
pub enum MeshPrimitive {
    Box { size: [f32; 3] },
    Cylinder { radius: f32, length: f32 },
    Capsule { radius: f32, length: f32 },
    Sphere { radius: f32 },
}

impl From<Cube> for Geometry {
    fn from(value: Cube) -> Self {
        return Geometry::Primitive(
            MeshPrimitive::Box { size: [value.size, value.size, value.size] }
        )
    }
}

// impl Into<Mesh> for Geometry {
//     fn into(self) -> Mesh {
//         match self {
//             Self::Primitive(variant)  => variant.into(),
//             Mesh => return shape::Cube{size: 1.0}.into(),
//         }
//     }
// }

impl Into<Mesh> for MeshPrimitive {
    fn into(self) -> Mesh {
        match self {
            Self::Box { size } => 
                shape::Box{
                    min_x: -size[0] * 0.5,
                    max_x: size[0] * 0.5,
                    min_y: -size[1] * 0.5,
                    max_y: size[1] * 0.5,
                    min_z: -size[2] * 0.5,
                    max_z: size[2] * 0.5,
                }.into(),
            Self::Cylinder { radius, length } => shape::Cylinder{radius: radius, height: length, ..default()}.into(),
            Self::Capsule { radius, length } => shape::Capsule{radius: radius, depth: length, ..default()}.into(),
            Self::Sphere { radius } => shape::Capsule{radius: radius, depth: 0.0, ..default()}.into()
        }
    }
}
