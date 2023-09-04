//use serde::{Deserialize, Serialize};
use std::path::Path;
use std::convert::From;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;








/// bundle for all things a singular model would need. More complex models will their own bundles
// #[derive(Bundle, Default)]
// pub struct ModelBundle {
//     pub pbr_bundle: PbrBundle,
//     pub physics: PhysicsBundle,
//     pub selection_enabler: MakeSelectableBundle,
//     pub serializer_type: SerializeType,
// }

// impl ModelBundle {
    
//     pub fn new(
//         mesh_handle: Handle<Mesh>,
//         material_handle: Handle<StandardMaterial>,
//         transform: Transform,
//         category: SerializeType,

//         // model_collision_groups: Option<CollisionGroups>,
//         // model_solver_groups: Option<SolverGroups>,
//     ) -> Self {
//         return Self {
//             pbr_bundle: PbrBundle {
//                 mesh: mesh_handle,
//                 material: material_handle,
//                 transform: transform,
//                 ..default()
//             },
//             physics: PhysicsBundle::default(),
//             selection_enabler: MakeSelectableBundle::default(),
//             serializer_type: category,
//             // model: ModelFlag {
//             //     geometry: geometry,
//             //     material: material,
//             //     transform: model_position,
//             //     ..default()

//             // },
//             // collision_groups: model_collision_groups.unwrap_or_default(),
//             // solver_groups: model_solver_groups.unwrap_or_default(),
//             ..default()
//         }
//     }
// }

/// collection of all things required for something to have "physics"
#[derive(Bundle)]
pub struct PhysicsBundle {
    /// rigid body type. Not setting this to `Dynamic`(I.E: a moving body) will probably cause errors.
    pub rigid_body: RigidBody, 
    /// Collider geometry. initialize this with Default() of ConvexDecomposition
    pub async_collider: AsyncCollider, 
    /// Mass of the robot(not sure what the mass is measured in?)
    pub mass: AdditionalMassProperties, 
    /// friction rules for object. No clue how this works, and this should probably be abstracted away from the user's eyes through a "Material" component/resource?
    pub friction: Friction,
    /// external forces being applied on a robot. These are not implied(except gravity?), and must be manually set on robot initialization.
    //external_forces: ExternalForce, 
    /// velocity of object. A model does not need this object to have a velocity, but `in order to read/write to the object's velocity, you need to have this object`
    pub velocity: Velocity,
    /// sets weather continous or discrete collision is the collision detection for this model. Continous = more accurate/more slow, discrete = faster/more innacurate
    pub continous_collision_setting: Ccd, 
    /// "for filtering what pair of colliders should have their contacts (or intersection test if at least one of the colliders is a sensor) computed by the narrow-phase. This filtering happens right after the broad-phase, at the beginning of the narrow phase."
    pub collision_groups: CollisionGroups,
    /// "A solver_groups for filtering what pair of colliders should have their contact forces computed. This filtering happens at the end of the narrow-phase, before the constraints solver"
    pub solver_groups: SolverGroups,
}

impl Default for PhysicsBundle {
    fn default() -> Self{
        Self {
            rigid_body: RigidBody::Dynamic,
            async_collider: AsyncCollider(ComputedColliderShape::ConvexDecomposition
                (
                    default()
                )),
            continous_collision_setting: Ccd::enabled(),
            mass: AdditionalMassProperties::Mass(1.0),
            friction: Friction { coefficient: (1000.0), combine_rule: (CoefficientCombineRule::Average) },
            // external_forces: ExternalForce { /// Can't think of a reason to use external force, commenting out for now.
            //     force: (Vec3::new(0.0, 0.0, 0.0)),
            //     torque: (Vec3::new(0.0, 0.0, 0.0))
            //     },
            velocity: Velocity{
                linvel: (Vec3::default()),
                angvel: (Vec3::default()), 
            },
            collision_groups: Default::default(),
            solver_groups: Default::default(),
        }
    }
}
/// Bundle that contains everything a model to simulate interacting with physics.
///
///  Use ```new()``` to initialize this more easily.
// #[derive(Bundle)]
// pub struct ModelBundle {
//     /// root model of robot. Stuff like wheels should probably attach to this. 
//     model : ModelFlag, 
//     //physics: PhysicsBundle,
//     // add bundle for making this model selectable
//     //selectable_bundle: MakeSelectableBundle,
//     // marks model as serializable by 
//     // save: Save,

// }

// impl Default for ModelBundle {
//     fn default() -> Self{
//         Self {
//             model: ModelFlag::default(),
//             //physics: PhysicsBundle::default(),
//             //selectable_bundle: MakeSelectableBundle::default(),
//         }
//     }
// }

// impl ModelBundle {
    
//     pub fn new(
//         geometry: Geometry,
//         model_position: Transform,
//         material: StandardMaterial,
//         // model_collision_groups: Option<CollisionGroups>,
//         // model_solver_groups: Option<SolverGroups>,
//     ) -> Self {
//         return Self {
//             model: ModelFlag {
//                 geometry: geometry,
//                 material: material,
//                 transform: model_position,
//                 ..default()

//             },
//             // collision_groups: model_collision_groups.unwrap_or_default(),
//             // solver_groups: model_solver_groups.unwrap_or_default(),
//             ..default()
//         }
//     }
// }

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Component))]
pub enum AssetSource {
    Local(String),
    Remote(String),
    Search(String),
    Bundled(String),
    Package(String),
}

impl AssetSource {
    pub fn label(&self) -> &str {
        match self {
            Self::Local(_) => "Local",
            Self::Remote(_) => "Remote",
            Self::Search(_) => "Search",
            Self::Bundled(_) => "Bundled",
            Self::Package(_) => "Package",
        }
    }
}

impl Default for AssetSource {
    fn default() -> Self {
        AssetSource::Local(String::new()).into()
    }
}

// Utility functions to add / strip prefixes for using AssetSource in AssetIo objects
impl From<&Path> for AssetSource {
    fn from(path: &Path) -> Self {
        if let Some(path) = path.to_str().and_then(|p| Some(String::from(p))) {
            AssetSource::from(&path)
        } else {
            AssetSource::default()
        }
    }
}

// Utility functions to add / strip prefixes for using AssetSource in AssetIo objects
impl From<&String> for AssetSource {
    fn from(path: &String) -> Self {
        // TODO(luca) pattern matching here would make sure unimplemented variants are a compile error
        if let Some(path) = path.strip_prefix("rmf-server://").map(|p| p.to_string()) {
            return AssetSource::Remote(path);
        } else if let Some(path) = path.strip_prefix("file://").map(|p| p.to_string()) {
            return AssetSource::Local(path);
        } else if let Some(path) = path.strip_prefix("search://").map(|p| p.to_string()) {
            return AssetSource::Search(path);
        } else if let Some(path) = path.strip_prefix("bundled://").map(|p| p.to_string()) {
            return AssetSource::Bundled(path);
        } else if let Some(path) = path.strip_prefix("package://").map(|p| p.to_string()) {
            return AssetSource::Package(path);
        }
        AssetSource::default()
    }
}

impl From<&AssetSource> for String {
    fn from(asset_source: &AssetSource) -> String {
        match asset_source {
            AssetSource::Remote(uri) => String::from("rmf-server://") + uri,
            AssetSource::Local(filename) => String::from("file://") + filename,
            AssetSource::Search(name) => String::from("search://") + name,
            AssetSource::Bundled(name) => String::from("bundled://") + name,
            AssetSource::Package(path) => /*String::from("package://") + */ path.to_owned(), //package part of papckage is not needed for now..
        }
    }
}