use bevy::prelude::*;
use bevy_asset_loader::prelude::AssetCollectionApp;
use bevy_common_assets::xml::XmlAssetPlugin;
use crate::body::robot::resources::*;
use crate::body::robot::systems::*;


pub const BOT_URDF: &str = "urdf/diff_bot.xml";

pub struct RobotPlugin;

impl Plugin for RobotPlugin {
    fn build(&self, app: &mut App){
        app
        //.add_plugin(XmlAssetPlugin::<Urdf>::new(&["xml"]))
        //.init_collection::<UrdfStuff>()
        .add_startup_system(spawn_robot)
        .add_startup_system(print_urdf)
        ;
    }
}