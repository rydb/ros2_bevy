mod body;
mod timers;
mod mesh;
mod editor;
mod urdf;
mod serialization;

use bevy::{prelude::*, reflect::TypePath, input::keyboard::KeyboardInput, tasks::IoTaskPool};
use std::{fs::File, io::Write};

use bevy_rapier3d::prelude::{RigidBody, GravityScale, ImpulseJoint};
use body::robot::{FeatureTestPlugin, RobotTestPlugin};
use bevy_flycam::prelude::*;
use bevy_mod_raycast::{
    print_intersections, DefaultPluginState, DefaultRaycastingPlugin, RaycastMesh, RaycastMethod,
    RaycastSource, RaycastSystem,
};
use editor::plugins::EditorPlugin;
//use crate::body::cube::components::*;

fn main() {
    App::new()
        .add_plugins(
            (
                DefaultPlugins,//< --- bevy needs these in order to run
                FeatureTestPlugin, // plugin which contains(mostly) everything program needs to run.
                NoCameraPlayerPlugin, // <-- Camera
                EditorPlugin,
                

            )
        )
        .add_systems(Update, serialize_world)
        .run();
}

const NEW_SCENE_FILE_PATH: &str = "scenes/load_scene_example-new.scn.ron";

/// marks component as a valid candidate for serialization. The serialization system will take entities marked with this, and attempt to serialize them into
/// a save file.
#[derive(Component)]
pub struct Serializable {

}

// take a world, serialize it to assets/scenes as a .ron file. 
pub fn serialize_world(
    world: &World,
    keys: Res<Input<KeyCode>>,


) {
    if keys.just_pressed(KeyCode::AltRight) {
        println!("serializing world");
        let scene = DynamicScene::from_world(&world);

        let type_registry = world.resource::<AppTypeRegistry>();
        
        let serialized_scene = scene.serialize_ron(type_registry);

        println!("serialized scene result is {:#?}", serialized_scene);

        // #[cfg(not(target_arch = "wasm32"))]
        // IoTaskPool::get()
        //     .spawn(async move {
        //         // Write the scene RON data to file
        //         File::create(format!("assets/{NEW_SCENE_FILE_PATH}"))
        //             .and_then(|mut file| file.write(serialized_scene.as_bytes()))
        //             .expect("Error while writing scene to file");
        //     })
        //     .detach();
    }

    //let type_registry = world.resource::<AppTypeRegistry>().clone();
    // scene_world.insert_resource(type_registry);

}