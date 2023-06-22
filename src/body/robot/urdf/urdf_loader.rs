//! urdf loarder for robots. Should create a
//! unique urdf resource for models to read from.

use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    utils::BoxedFuture,
};

use serde::Deserialize;

use urdf_rs::Robot;
use std::collections::HashMap;

use super::urdf_to_bevy::{UrdfRoot};

use thiserror::Error;

use std::prelude::*;

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

#[derive(Resource, Default)]
pub struct SpawnableRobots {
    pub list: Vec<BevyRobot>,
}

#[derive(Component, TypeUuid)]
#[uuid = "6f49513d-eec8-4d23-ab5c-812fbcafe738"]
pub struct BevyRobot {
    pub urdf_file: Handle<UrdfRoot>,
    // pub root_folder: String,
    // pub source_folder: String,
    // pub package_folder: String,
    // pub state: LoadState,
}



impl BevyRobot {

    /// create a strong mesh handle based on the source of a mesh.
    pub fn fetch_mesh_from_source(self, mut asset_server: Res<AssetServer>, mut mesh_server: Res<Assets<Mesh>> , mesh_source: AssetSource) -> Handle<Mesh>{
        match mesh_source {
            AssetSource::Package(filename) => 
             {
                let loaded_mesh: Handle<Mesh> = asset_server.load(
                    self.root_folder + &self.source_folder + &self.package_folder + &filename
                );
                return loaded_mesh;
            }, 
            _ => panic!("fetching mesh from, {:#?}, isn't implemented. Add an implementation to remove this error", mesh_source),
        }
    }
}

/// Takes BevyRobots, adds urdfs to those robots, so that a robot spawner can spawn those
/// if you want to spawn a robot, initialize a BevyRobot in here, and push it to 
/// the robots list. The robot spawner system will read that list and spawn bevy robots
/// based on that list. 
pub fn stage_robots_to_initialize(mut robots: ResMut<SpawnableRobots>, asset_server: Res<AssetServer>) {    
    // add urdfs you want to be loaded here
    let diff_bot = BevyRobot {
        urdf_file: asset_server.load(
            "group_robot_ros2/urdf/cube.xml"
        ),
        // root_folder: "/group_robot_ros2".to_owned(),
        // source_folder: "/src".to_owned(),
        // package_folder: "/model".to_owned(),
    };
    robots.list.push(diff_bot)

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