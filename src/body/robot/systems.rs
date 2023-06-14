use std::time::Duration;

use bevy::prelude::*;
use bevy::asset::FileAssetIo;
use bevy_rapier3d::prelude::*;


use crate::body::robot::{components::*, self};

use super::{resources::CountDownTimer, urdf::{urdf_to_bevy::UrdfRoot, urdf_loader::SpawnedRobot}};

/// NOTE: NAME OF BEVY ASSET FOLDER. SHOULD BE REPLACED BY PROPER ASSET LOADER LATER.
pub const ASSET_FOLDER: &str = "assets/";

/// Source dir after reaching into the robot's root folder which holds packages. keeping as const till a better idea comes up.
pub const SOURCE_FOLDER_FOR_ROBOT: &str = "src/";

// dir for urdf folder. 
pub const URDF_FOLDER: &str = "urdf/";/// A Robotics framework in the abstract 

// used to donote spawned model is a "part". Used to check
// for any models that the part is "bound" to.
#[derive(Component)]
pub struct Part;


/// get all robots, and spawn them from their urdfs
pub fn spawn_robots_from_urdf(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut asset_server: Res<AssetServer>,

    mut model_query: Query<&Robot>,
) {
    //fetch all entities which are robots, and spawn them from their urdfs.
    for robot in model_query.iter_mut() {
        // get path for urdf(hard coded until I figure out how asset loaders work?)
        let urdf_folder_path = 
        FileAssetIo::get_base_path() //path to root dir of project
        .join(
        ASSET_FOLDER.to_owned() //assets dir
            + &robot.root_dir //robot dir
            + SOURCE_FOLDER_FOR_ROBOT //src dir
            + &robot.pkg_dir // pkg dir
            + URDF_FOLDER //urdf dir
            + &robot.urdf_file) // urdf file + extension
        .to_str()
        .unwrap()
        .to_owned();

        println!("{:#?}", urdf_folder_path);

        let robot_urdf = urdf_rs::read_file(urdf_folder_path).unwrap();

        println!("getting model...");
        for links in robot_urdf.links {
            //println!("LINK: {:#?}");
            for visual in links.visual {
                //println!("{:#?} ", visual)
                match visual.geometry {
                    urdf_rs::Geometry::Box { size } => println!("box detected!"),
                    urdf_rs::Geometry::Cylinder { radius, length } => println!("Cylinder detected!"),
                    urdf_rs::Geometry::Capsule { radius, length } => println!("Mesh detected!"),
                    urdf_rs::Geometry::Sphere { radius } => println!("Sphere detected"),
                    urdf_rs::Geometry::Mesh { filename, scale } => {
                        let scale = scale
                            .clone()
                            .and_then(|s| Some(Vec3::from_array(
                                s.map(|v| v as f32)
                            )));
                        //println!("package type + file name of mesh is: {:#}", filename);

                        let mesh_path = robot.root_dir.clone() + SOURCE_FOLDER_FOR_ROBOT + &String::from(&AssetSource::from(&filename));
                        
                        //get urdf path urdf file
                        println!("mesh file path is: {:#?}", mesh_path );
                        
                        //spawn geometry
                        let mesh_handle: Handle<Mesh> = asset_server.load(mesh_path);
                        let model = commands.spawn(

                            (
                                    PbrBundle {
                                        mesh: mesh_handle,
                                        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                                        transform: Transform::from_xyz(0.0, 5.0, 0.0),
                                        ..default()
                                    },
                                    //physics properties
                                    RigidBody::Dynamic, // set this object to be a dynamic rigid body
                                    AsyncCollider(ComputedColliderShape::ConvexDecomposition
                                        (
                                            default()
                                        )
                                    ),
                                    Part {}
                                )
                            ).id();
                        println!("spawned: {:#?}", model);


                    }//println!("Mesh detected!"),
                
                }
            }    
        }

    }
    
    //println!("Joints for robot {:#?}", robot_urdf.joints)
}



///TODO
/// 1. ADD ROBOT SPAWNING
/// 2. ADD ROBOT MOVING

pub fn spawn_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assset_server: Res<AssetServer>,

    model_query: Query<Entity, With<Robot>>,
) {
    let cube = commands.spawn(
        ModelBundle::new(
            meshes.add(Mesh::from(shape::Cube {size: 1.0})),
            Transform::from_xyz(0.0, 0.0, 0.0),
        )

            
    ).id();

}

            // robot: Robot {
            //     name: "diff_bot".to_owned(),
            //     root_dir: "group_robot_ros2/".to_owned(),
            //     pkg_dir: "model_pkg".to_owned(),
            //     urdf_file: "diff_bot.xml".to_owned(),
            // },

pub fn list_robots (
    model_query: Query<&Robot>,
) {
    //println!("current robots are: ");
    for robot in model_query.iter() {
        //println!("{:#?}", robot.name)
    }
}
pub fn setup_diff_bot(
    mut urdf: ResMut<SpawnedRobot>,
    asset_server: Res<AssetServer>
) {
    urdf.handle = asset_server.load("diff_bot.xml");
    println!("urdf is {:#?}", urdf)
}

/// Predicted steps:
/// 1. load urdf
/// 2. load all sub models
/// 3.
pub fn load_diff_bot(
    mut urdf_state: ResMut<SpawnedRobot>,
    urdf_assets: ResMut<Assets<UrdfRoot>>) {
    
    let urdf = urdf_assets.get(&urdf_state.handle); // urdf.handle = asset_server.load("diff_bot.xml");

    println!("urdf is: {:#?}", urdf.unwrap())


}


// moves all robots forward(knowing the total forces being exerted on the collider would be helpful? Mabye for establishing some kind of formula?)
pub fn move_robot_forward(
    mut model_query: Query<&mut ExternalForce, With<Robot>>,
    mut timer_query: ResMut<CountDownTimer>,
    time: Res<Time>,
    
) {
    // for mut robot in model_query.iter_mut() {
    //     println!("timer is currently at {:#?}", timer_query.timer.elapsed_secs());
    //     println!("time passed on timer: {:#?}" , timer_query.timer.elapsed());
    //     timer_query.timer.tick(time.delta());
    //     robot.force = Vec3::new(100.0 * timer_query.timer
    //     .percent_left(),
    //     0.0, 10.0);

    //     println!("remaining external force on robot is {:#?}", robot.force)
    // }
}
