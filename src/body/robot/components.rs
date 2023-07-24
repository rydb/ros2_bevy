use bevy_mod_raycast::RaycastMesh;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::convert::From;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;



// denotes entity is "wheel", for sending drive instructions.
#[derive(Component)]
pub struct Wheel {}


/// Bundle that contains everything a model to simulate interacting with physics.
///
///  Use ```new()``` to initialize this more easily.
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
    //external_forces: ExternalForce, 
    /// velocity of object. A model does not need this object to have a velocity, but `in order to read/write to the object's velocity, you need to have this object`
    velocity: Velocity,
    /// sets weather continous or discrete collision is the collision detection for this model. Continous = more accurate/more slow, discrete = faster/more innacurate
    continous_collision_setting: Ccd, 
    /// "for filtering what pair of colliders should have their contacts (or intersection test if at least one of the colliders is a sensor) computed by the narrow-phase. This filtering happens right after the broad-phase, at the beginning of the narrow phase."
    collision_groups: CollisionGroups,
    /// "A solver_groups for filtering what pair of colliders should have their contact forces computed. This filtering happens at the end of the narrow-phase, before the constraints solver"
    solver_groups: SolverGroups,
    /// raycast mesh for getting selected by raycasts that look for rigid bodies. Should be left initialized by ..default()
    raycast_for_rigidbody: RaycastMesh<RigidBody>,
    // weather rigid body is selected    
}

impl Default for ModelBundle {
    fn default() -> Self{
        Self {
            model: PbrBundle::default(),
            rigid_body: RigidBody::Dynamic,
            async_collider: AsyncCollider(ComputedColliderShape::ConvexDecomposition
                (
                    default()
                )),
            continous_collision_setting: Ccd::enabled(),
            mass: AdditionalMassProperties::Mass(1.0),
            friction: Friction { coefficient: (99.0), combine_rule: (CoefficientCombineRule::Average) },
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
            raycast_for_rigidbody: RaycastMesh::<RigidBody>::default(),
        }
    }
}

impl ModelBundle {
    
    pub fn new(
        mesh_handle: Handle<Mesh>,
        model_position: Transform,
        material_handle: Handle<StandardMaterial>,
        // model_collision_groups: Option<CollisionGroups>,
        // model_solver_groups: Option<SolverGroups>,
    ) -> Self {
        return Self {
            model: PbrBundle {
                mesh: mesh_handle,
                material: material_handle,
                transform: model_position,
                ..default()

            },
            // collision_groups: model_collision_groups.unwrap_or_default(),
            // solver_groups: model_solver_groups.unwrap_or_default(),
            ..default()
        }
    }
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