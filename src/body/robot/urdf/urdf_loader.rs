//! urdf loarder for robots. Should create a
//! unique urdf resource for models to read from.

use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    utils::BoxedFuture,
};

// use serde::Deserialize;

// use urdf_rs::Robot;
// use std::collections::HashMap;


//use crate::body::robot::components::ModelBundle;

use super::urdf_to_bevy::{UrdfRoot};

use thiserror::Error;

//use std::prelude::*;

use bevy_asset_loader::prelude::*;

#[derive(Default)]
pub struct UrdfLoader;

impl AssetLoader for UrdfLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move { Ok(load_urdf(bytes, load_context).await?) })
    }

    fn extensions(&self) -> &[&str] {
        static EXTENSIONS: &[&str] = &["xml"];
        EXTENSIONS
    }
}

#[derive(Error, Debug)]
pub enum UrdfError {
    #[error("Failed to load Urdf")]
    ParsingError,
    //Io(#[from] std::io::Error),
}

/// Weather this urdf is loaded or not. 
#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum LoadState {
    #[default]
    Unloaded,
    Loaded,
}

// /// Robot that is spawned from UrdfRoot
// #[derive(Resource, Default, Debug, AssetCollection)]
// pub struct SpawnedRobot {
//     #[asset(path = "cube.xml")]
//     pub urdf_handle: Handle<UrdfRoot>,
    
// }

#[derive(Resource, Default, AssetCollection)]
pub struct SpawnableRobots {
    #[asset]
    pub list: Vec<BevyRobot>,
}

#[derive(Component, TypeUuid)]
#[uuid = "6f49513d-eec8-4d23-ab5c-812fbcafe738"]
pub struct BevyRobot {
    pub urdf_file: Handle<UrdfRoot>,
    /// urdf path starting from the assets/, folder. E.G:, if urdf is in robots/ project, this should start
    /// from robots/
    //pub urdf_path: String,
    pub models_dir_path: String,
    //pub state: LoadState,
}

impl BevyRobot {

    // pub fn new(asset_server: Res<AssetServer>, urdf_path_through_assets: String) -> Self{
        
    //     // folder for this should be organized as so:
    //     // root folder / source_folder(e.g: /src) / package_folder / urdf_folder / urdf.xml
    //     // let folders: Vec<&str> =  urdf_path_through_assets.split("/").collect();
    //     // return Self {
    //     //     urdf_file: asset_server.load(urdf_path_through_assets),
    //     //     urdf_path: urdf_path_through_assets,
    //     //     models_folder: 

    //     // }
    // }

    // infer mesh folder from urdf path.
    // mesh folder should be in the `/models` for local/package model locations. 
    // pub fn mesh_dir_from_urdf(self) {
        
    // }
    // create a strong mesh handle based on the source of a mesh.
    // pub fn fetch_mesh_from_source(self, asset_server: Res<AssetServer>, mut mesh_server: Res<Assets<Mesh>> , mesh_source: AssetSource) -> Handle<Mesh>{
    //     match mesh_source {
    //         AssetSource::Package(filename) => 
    //          {
    //             let loaded_mesh: Handle<Mesh> = asset_server.load(
    //                 self.root_folder + &self.source_folder + &self.package_folder + &filename
    //             );
    //             return loaded_mesh;
    //         }, 
    //         _ => panic!("fetching mesh from, {:#?}, isn't implemented. Add an implementation to remove this error", mesh_source),
    //     }
    // }
}




/// stages bevy robots to be spawned in the world by a later spawner function

pub fn stage_robots_to_spawn_from_urdf(
    mut robots: ResMut<SpawnableRobots>,
    asset_server: Res<AssetServer>,
    urdf_server: Res<Assets<UrdfRoot>>,
    mut commands: Commands,) {    
    // add urdfs you want to be loaded here
    let robot_pkg_path = "group_robot_ros2/src/model_pkg/";
    let diff_bot = BevyRobot {
        urdf_file: 
            asset_server.load(robot_pkg_path.to_owned() + "urdf/cube.xml"),
        models_dir_path: robot_pkg_path.to_owned() + "models/"
    };
    commands.spawn(diff_bot);
}

async fn load_urdf<'a, 'b>(
    bytes: &'a [u8],
    load_context: &'a mut LoadContext<'b>,
) -> Result<(), UrdfError> {
    if let Some(res) = std::str::from_utf8(bytes)
        .ok()
        .and_then(|utf| urdf_rs::read_from_string(utf).ok())
    {
        load_context.set_default_asset(LoadedAsset::new(UrdfRoot(res)));
        return Ok(());
    } else {
        return Err(UrdfError::ParsingError);
    }
}