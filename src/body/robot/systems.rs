use bevy::prelude::*;
use bevy::asset::FileAssetIo;
use bevy_rapier3d::prelude::*;


use crate::body::robot::{components::*, self};

/// NOTE: NAME OF BEVY ASSET FOLDER. SHOULD BE REPLACED BY PROPER ASSET LOADER LATER.
pub const ASSET_FOLDER: &str = "assets/";

/// Source dir after reaching into the robot's root folder which holds packages. keeping as const till a better idea comes up.
pub const SOURCE_FOLDER_FOR_ROBOT: &str = "src/";

// dir for urdf folder. 
pub const URDF_FOLDER: &str = "urdf/";

/// A Robotics framework in the abstract 
struct ROS2 {
    /// root directoy of robot. e.g, `"diff_bot/"`
    robot_root_dir: String,
    /// urdf file + extension, e.g: `"diff_bot.xml"`
    urdf_file: String,
    /// where info relevant to the robot is stored, e.g: if robot information is in src/model_pkg/, then its `"model_pkg/"`
    pkg_dir: String,
}


// used to donote spawned model is a "part". Used to check
// for any models that the part is "bound" to.
#[derive(Component)]
pub struct Part;

// impl Robotics_Framework for ROS2 {

//     fn list_models(
//         &self,
//         mut commands: Commands,
//         meshes: ResMut<Assets<Mesh>>,
//         mut materials: ResMut<Assets<StandardMaterial>>,
//         asset_server: Res<AssetServer>,

//         mut model_query: Query<Entity, With<Part>>,
//     ) {
//         println!("listing models..");
//         for detected_model in &model_query {
//             println!("sucessfully spawned: {:#?}", detected_model)
//         }
//     }
// }





// ///get model from urdf
// fn get_model(
//     mut commands: Commands,
//     meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
//     asset_server: Res<AssetServer>,

//     mut model_query: Query<Entity, With<Robot>>,
// ) {
//     // get path for urdf.
//     let urdf_folder_path = 
//     FileAssetIo::get_base_path() //path to root dir of project
//     .join(
//     ASSET_FOLDER.to_owned() //assets dir
//         + &self.robot_root_dir //robot dir
//         + SOURCE_FOLDER_FOR_ROBOT //src dir
//         + &self.pkg_dir // pkg dir
//         + URDF_FOLDER //urdf dir
//         + &self.urdf_file) // urdf file + extension
//     .to_str()
//     .unwrap()
//     .to_owned();

//     println!("{:#?}", urdf_folder_path);

//     let robot_urdf = urdf_rs::read_file(urdf_folder_path).unwrap();

//     println!("getting model...");
//     for links in robot_urdf.links {
//         //println!("LINK: {:#?}");
//         for visual in links.visual {
//             //println!("{:#?} ", visual)
//             match visual.geometry {
//                 urdf_rs::Geometry::Box { size } => println!("box detected!"),
//                 urdf_rs::Geometry::Cylinder { radius, length } => println!("Cylinder detected!"),
//                 urdf_rs::Geometry::Capsule { radius, length } => println!("Mesh detected!"),
//                 urdf_rs::Geometry::Sphere { radius } => println!("Sphere detected"),
//                 urdf_rs::Geometry::Mesh { filename, scale } => {
//                     let scale = scale
//                         .clone()
//                         .and_then(|s| Some(Vec3::from_array(
//                             s.map(|v| v as f32)
//                         )));
//                     //println!("package type + file name of mesh is: {:#}", filename);

//                     let mesh_path = self.robot_root_dir.clone() + SOURCE_FOLDER_FOR_ROBOT + &String::from(&AssetSource::from(&filename));
                    
//                     //get urdf path urdf file
//                     println!("mesh file path is: {:#?}", mesh_path );
                    
//                     //spawn geometry
//                     let mesh_handle: Handle<Mesh> = asset_server.load(mesh_path);
//                     let model = commands.spawn(

//                         (
//                                 PbrBundle {
//                                     mesh: mesh_handle,
//                                     material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
//                                     transform: Transform::from_xyz(0.0, 5.0, 0.0),
//                                     ..default()
//                                 },
//                                 //physics properties
//                                 RigidBody::Dynamic, // set this object to be a dynamic rigid body
//                                 AsyncCollider(ComputedColliderShape::ConvexDecomposition
//                                     (
//                                         default()
//                                     )
//                                 ),
//                                 Part {}
//                             )
//                         ).id();
//                     println!("spawned: {:#?}", model);


//                 }//println!("Mesh detected!"),
            
//             }
//         }    
//     }

//     //println!("Joints for robot {:#?}", robot_urdf.joints)
// }

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
        }
    }
}


#[derive(Bundle)]
pub struct RobotBundle {
    /// model that the robot originates from.
    root_model: ModelBundle,

    /// robot struct. Anything related to the robot that is tied to the robot it self. Also used to identify non robot model from robot models.
    robot: Robot,
}

#[derive(Component)]
pub struct Robot {
    name: String,
    
    // The root component/head of the robot
    // root_body: (
    //     PbrBundle,
    //     RigidBody,
    //     AsyncCollider,
    // ),

}

///TODO
/// 1. ADD ROBOT SPAWNING
/// 2. ADD ROBOT MOVING

pub fn spawn_robot_from_urdf(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assset_server: Res<AssetServer>,

    model_query: Query<Entity, With<Robot>>,
) {
    let diff_bot = commands.spawn(
        RobotBundle {
            root_model: ModelBundle::new(
                meshes.add(Mesh::from(shape::Cube {size: 1.0})),
                Transform::from_xyz(0.0, 0.0, 0.0),
            ),
            robot: Robot {name: "diff_bot".to_owned()},
            
        }
    )
    .id();

}

pub fn list_robots (
    model_query: Query<&Robot>,
) {
    println!("current robots are: ");
    for robot in model_query.iter() {
        println!("{:#?}", robot.name)
    }
}



// moves all robots forward
pub fn move_robot_forward(
    mut model_query: Query<&mut ExternalForce, With<Robot>>,
) {
    for mut robot in model_query.iter_mut() {
        println!("moving {:#?}", robot);
        robot.force = Vec3::new(1000.0, 0.0, 10.0);
    }
}
    // mut ext_forces: Query<&mut ExternalForce>,
    // mut ext_impulses: Query<&mut ExternalImpulse>) {
    
    
    // // Apply forces.
    // for mut ext_force in ext_forces.iter_mut() {
    //     ext_force.force = Vec3::new(1000.0, 2000.0, 300000000000000000000.0);
    //     ext_force.torque = Vec3::new(0.4, 0.5, 0.6);
    // }

    // // Apply impulses.
    // for mut ext_impulse in ext_impulses.iter_mut() {
    //     ext_impulse.impulse = Vec3::new(100.0, 200.0, 3000000000000000.0);
    //     ext_impulse.torque_impulse = Vec3::new(0.4, 0.5, 0.6);
    // }
    // println!("applied external forces to robot. External forces on robot are now: {:#?} ", ext_forces)
