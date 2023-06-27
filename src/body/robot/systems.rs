use std::time::Duration;

use bevy::prelude::*;
use bevy::asset::FileAssetIo;
use bevy_rapier3d::prelude::*;



use crate::body::robot::{components::*, BevyRobot};

use super::{resources::CountDownTimer, urdf::{urdf_to_bevy::UrdfRoot, urdf_loader::SpawnableRobots}};

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





pub fn spawn_urdf_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut robots: Query<Entity, &BevyRobot>
) {
    for robot in robots.iter() {
        println!("spawning robot");
        commands.spawn(ModelBundle::new(
            meshes.add(Mesh::from(shape::Cube {size: 1.0})),
            Transform::from_xyz(0.0, 0.0, 0.0),
        ));
    }
}

///TODO
/// 1. ADD ROBOT SPAWNING
/// 2. ADD ROBOT MOVING


pub fn spawn_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    //mut materials: ResMut<Assets<StandardMaterial>>,
    //assset_server: Res<AssetServer>,

    //model_query: Query<Entity, With<BevyRobot>>,
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
    model_query: Query<&BevyRobot>,
) {
    //println!("current robots are: ");
    for robot in model_query.iter() {
        
        //println!("{:#?}", robot.name)
    }
}



/// Predicted steps:
/// 1. load urdf
/// 2. load all sub models
/// 3.
// pub fn load_diff_bot(
//     mut commands: Commands,
//     assets: Res<SpawnableRobots>,
//     // mut urdf_state: ResMut<SpawnedRobot>,
//     //asset_server: Res<AssetServer>,
//     urdf_assets: ResMut<Assets<UrdfRoot>>
// ) {
//     //check to see if we can fetch urdf, if we can, proceed
//     match urdf_assets.get(&assets.urdf_handle) {
//         Some(urdf) => commands.spawn
//         (
//     (
//                 BevyRobot {name: "diff_bot".to_owned(), root_dir: "group_robot_ros2".to_owned(), pkg_dir: "model_pkg".to_owned()},
//                 urdf.to_owned(),
//             )    
            
//         )/*println!("urdf is {:#?}", urdf)*/,
//         None => panic!("Failed to fetch urdf. Unable to retrieve urdf from handle, {:#?}", &assets.urdf_handle)
//     };
    
//     // match urdf {
//     //     Some(v) => println!("urdf is {:#?}", urdf.unwrap()),
//     //     None => panic!("Failed to fetch urdf. Unable to retrieve urdf from the handle, {:#?} ", urdf_state.handle)
//     // }
//     //println!("urdf is: {:#?}", urdf.())


// }


// moves all robots forward(knowing the total forces being exerted on the collider would be helpful? Mabye for establishing some kind of formula?)
pub fn move_robot_forward(
    mut model_query: Query<&mut ExternalForce, With<BevyRobot>>,
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
