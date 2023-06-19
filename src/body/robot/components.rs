use serde::{Deserialize, Serialize};
use std::path::Path;
use std::convert::From;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;


/// Bundle that contains everything for a model that interacts with the physical world.
#[derive(Bundle)]
pub struct ModelBundle {
    /// root model of robot. Stuff like wheels should probably attach to this. 
    model : PbrBundle, 
    /// rigid body type. Not setting this to `Dynamic`(I.E: a moving body) will probably cause errors.
    rigid_body: RigidBody, 
    /// Collider geometry. initialize this with Default() of ConvexDecomposition
    async_collider: AsyncCollider, 
    /// Mass of the robot(not sure what the mass is measured in?)
    mass: AdditionalMassProperties, 
    /// friction rules for object. No clue how this works, and this should probably be abstracted away from the user's eyes through a "Material" component/resource?
    friction: Friction,
    /// external forces being applied on a robot. These are not implied(except gravity?), and must be manually set on robot initialization.
    external_forces: ExternalForce, 
    /// velocity of object. A model does not need this object to have a velocity, but `in order to read/write to the object's velocity, you need to have this object`
    velocity: Velocity,

}

impl ModelBundle {
    pub fn new(
        mesh_handle: Handle<Mesh>,
        model_position: Transform,
    ) -> Self {
        return Self {
            model: PbrBundle {
                mesh: mesh_handle,
                material: default(),
                transform: model_position,
                ..default()

            },
            rigid_body: RigidBody::Dynamic,
            async_collider: AsyncCollider(ComputedColliderShape::ConvexDecomposition
            (
                default()
            )),
            
            mass: AdditionalMassProperties::Mass(1.0),
            friction: Friction { coefficient: (1.0), combine_rule: (CoefficientCombineRule::Average) },
            external_forces: ExternalForce {
                force: (Vec3::new(0.0, 0.0, 0.0)),
                torque: (Vec3::new(0.0, 0.0, 0.0))
                },
            velocity: Velocity{
                linvel: (Vec3::default()),
                angvel: (Vec3::default()), 
            },
        }
    }
}




// #[derive(Bundle)]
// pub struct RobotBundle {
//     /// model that the robot originates from.
//     pub root_model: ModelBundle,
//     /// robot struct. Anything related to the robot that is tied to the robot it self. Also used to identify non robot model from robot models.
//     pub robot: Robot,
// }

#[derive(Component)]
pub struct BevyRobot {
    /// name of robot
    pub name: String,
    /// robot package root dir.
    pub root_dir: String,
    /// package which info of robot like urdfs/models, etc.. is stored in
    pub pkg_dir: String,
}


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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