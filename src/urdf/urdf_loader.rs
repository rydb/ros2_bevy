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

use crate::body::robot;

use super::urdf_to_bevy::UrdfRoot;

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

#[derive(Component, TypeUuid, Default, Reflect)]
#[reflect(Component)]
#[uuid = "6f49513d-eec8-4d23-ab5c-812fbcafe738"]
pub struct BevyRobot {
    pub urdf_file: Handle<UrdfRoot>,
    pub urdf_path: String,
    /// urdf path starting from the assets/, folder. E.G:, if urdf is in robots/ project, this should start
    /// from robots/
    //pub urdf_path: String,
    pub models_dir_path: String,
    //pub state: LoadState,
}

impl BevyRobot {

}




/// stages bevy robots to be spawned in the world by a later spawner function

pub fn stage_robots_to_spawn_from_urdf(
    //mut robots: ResMut<SpawnableRobots>,
    asset_server: Res<AssetServer>,
    //urdf_server: Res<Assets<UrdfRoot>>,
    mut commands: Commands,) {    
    // add urdfs you want to be loaded here
    let robot_pkg_path = "example_bot/src/model_pkg/";
    let urdf_path = robot_pkg_path.to_owned() + "urdf/diff_bot.xml";
    let diff_bot = BevyRobot {
        urdf_file: 
            asset_server.load(urdf_path.clone()),
        models_dir_path: robot_pkg_path.to_owned() + "models/",
        urdf_path: urdf_path
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