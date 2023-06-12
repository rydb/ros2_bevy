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
pub trait Robotics_Framework {
    fn get_model(
        &self,
        commands: Commands,
        meshes: ResMut<Assets<Mesh>>,
        materials: ResMut<Assets<StandardMaterial>>,
        assset_server: Res<AssetServer>,
    
        model_query: Query<Entity, With<Part>>,
    );
    fn list_models(
        &self,
        commands: Commands,
        meshes: ResMut<Assets<Mesh>>,
        materials: ResMut<Assets<StandardMaterial>>,
        assset_server: Res<AssetServer>,
    
        model_query: Query<Entity, With<Part>>,
    );


}
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

impl Robotics_Framework for ROS2 {
    ///get model from urdf
    fn get_model(
        &self,
        mut commands: Commands,
        meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        asset_server: Res<AssetServer>,

        mut model_query: Query<Entity, With<Part>>,
    ) {
        // get path for urdf.
        let urdf_folder_path = 
        FileAssetIo::get_base_path() //path to root dir of project
        .join(
      ASSET_FOLDER.to_owned() //assets dir
         + &self.robot_root_dir //robot dir
         + SOURCE_FOLDER_FOR_ROBOT //src dir
         + &self.pkg_dir // pkg dir
         + URDF_FOLDER //urdf dir
         + &self.urdf_file) // urdf file + extension
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
    
                        let mesh_path = self.robot_root_dir.clone() + SOURCE_FOLDER_FOR_ROBOT + &String::from(&AssetSource::from(&filename));
                        
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

        //println!("Joints for robot {:#?}", robot_urdf.joints)
    }
    fn list_models(
        &self,
        mut commands: Commands,
        meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        asset_server: Res<AssetServer>,

        mut model_query: Query<Entity, With<Part>>,
    ) {
        println!("listing models..");
        for detected_model in &model_query {
            println!("sucessfully spawned: {:#?}", detected_model)
        }
    }
}


pub struct Robot<T> where
    T: Robotics_Framework

    {
    //robotics framework used for the robot. E.G: ROS2
    framework: T,
}

///read robot urdf, and spawn robot from urdf
pub fn spawn_robot_from_urdf(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    assset_server: Res<AssetServer>,

    model_query: Query<Entity, With<Part>>,
) {
    let diff_bot = commands.spawn(
        (
            Robot {}
        )
    );

    /* OOP, BAD, RIGID, NOT EDITABLE -V
    let diff_bot = Robot {
        framework: 
            ROS2 {robot_root_dir: "group_robot_ros2/".to_string(), pkg_dir: "model_pkg/".to_string(), urdf_file: "diff_bot.xml".to_string()}
    };
    
    // test urdf stuff
    diff_bot.framework.get_model(commands, meshes, materials, assset_server, model_query);*/

}