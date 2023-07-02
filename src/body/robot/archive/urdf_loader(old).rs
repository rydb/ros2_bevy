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

use super::urdf_to_bevy::{UrdfRoot};

use thiserror::Error;

pub const TEST_FOLDER: String = "bob/".to_owned();

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

pub const FOLDER_PATH: String = "folder".to_owned();

/// Robot that is spawned from UrdfRoot
#[derive(Resource, Default, Debug, AssetCollection)]
pub struct SpawnedRobot {
    #[asset(path = (FOLDER_PATH + "folder/cube.xml"))]
    pub urdf_handle: Handle<UrdfRoot>,
    
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