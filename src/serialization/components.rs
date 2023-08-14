use bevy::prelude::*;
/// flags geometry type this model has. Should only be set once and left unedited. 
#[derive(Component, Reflect)]
//#[reflect(from_reflect = false)]
#[reflect(Component)]
pub enum GeometryFlag {
    Primitive(MeshPrimitive),
    Mesh {
        filename: String,
        scale: Vec3,
    },
}

/// Reflect, and Serialization both require a default implementation of structs. The default GeometryFlag resorts to an "fallback" mesh to
/// represent failed load attempts. (TODO): add a system that picks up error meshes, and displays them somewhere.
impl Default for GeometryFlag {
    fn default() -> Self {
        Self::Mesh {
            filename: "fallback.gltf".to_string(),
            scale: Vec3::new(0.0,0.0,0.0),
        }        
    }
}

#[derive(Debug, Clone, PartialEq, Reflect)]
#[derive(Component)]
pub enum MeshPrimitive {
    Box { size: [f32; 3] },
    Cylinder { radius: f32, length: f32 },
    Capsule { radius: f32, length: f32 },
    Sphere { radius: f32 },
}