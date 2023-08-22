use bevy::prelude::*;
use moonshine_save::prelude::Unload;
use moonshine_save::save::Save;

use crate::{body::robot::components::{PhysicsBundle, MakeSelectableBundle}, urdf::urdf_to_bevy::UrdfRoot};
use bevy::ecs::query::ReadOnlyWorldQuery;
use super::components::Geometry;
use crate::body::robot::components::Selected;

use moonshine_save::save::*;
use super::components::*;
use std::path::PathBuf;


/// collect entities with seriazer flags for multi part models, and add their non serializable
/// equivilent component so that their spawning systems can spawn them.
// pub fn spawn_multipart_models(
//     commands: Commands,
//     multipart_model_query: Query<(Entity, &SerializeType)>,
// ) {
//     for (e, serialize_type) in multipart_model_query.iter() {
//         match *serialize_type {
//             SerializeType::Skip => println!("found model marked to skip serializing, skipping"),
//             SerializeType::SingleModel
//         }
//     }
// }

/// collect entities with `ModelFlag` that don't have meshes, and spawn their meshes.  
pub fn spawn_models(
    unspawned_models_query: Query<(Entity, &ModelFlag), Without<Handle<Mesh>>>,
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assset_server: Res<AssetServer>,
) {
    for (e, model) in unspawned_models_query.iter() {
        let mesh_handle = match model.geometry.clone() {
            Geometry::Primitive(variant) => meshes.add(variant.into()), 
            Geometry::Mesh { filename, .. } => {
                println!("attempting to load mesh: {:#?}", filename);
                assset_server.load(filename)}
        };
        let material_handle = materials.add(model.material.clone());
        let trans = Transform::from(model.transform);
        // add all components a deserialized model needs to be useful. 
        commands.entity(e).insert(
            (
            PbrBundle {
                mesh: mesh_handle,
                material: material_handle,
                transform: trans,
                ..default()
            }, // add meshd
            PhysicsBundle::default(),// adds physics
            MakeSelectableBundle::default(), // makes model selectable 
            Unload, // marks entity to unload on deserialize
        )
        )
        // remove model flag 
        //.remove::<ModelFlag>()
        ;
        

    }
}

pub fn check_for_save_keypress(
    keys: Res<Input<KeyCode>>,
) -> bool{
    if keys.just_pressed(KeyCode::AltRight) {
        return true
    } else {
        return false
    }
}

pub fn check_for_load_keypress(
    keys: Res<Input<KeyCode>>,
) -> bool{
    if keys.just_pressed(KeyCode::AltLeft) {
        return true
    } else {
        return false
    }
}


pub fn save_into_file(path: impl Into<PathBuf>) -> SavePipeline {
    save::<With<SerializeType>>
        .pipe(into_file(path.into()))
        .pipe(finish)
        .in_set(SaveSet::Save)
}

pub fn save<Filter: ReadOnlyWorldQuery>(
    world: &World,
    serializable_querry: Query<Entity, Filter>,
    //serializable_querry: Query<(Entity, &)>,
    //robot_model_querry: Query<Entity, With<UrdfRoot>>,
    //mut commands: Commands,
) -> Saved {
    let mut builder = DynamicSceneBuilder::from_world(world);
    // block all types, and then add types that should be serialized here
    builder.deny_all();
    builder.allow::<Transform>();
    builder.allow::<SerializeType>();
    builder.allow::<ModelFlag>();

    builder.extract_entities(serializable_querry.iter());
    let scene = builder.build();
    Saved { scene }
}

//pub fn save_models()

// fn main() {
//     App::new()
//         .add_plugins(DefaultPlugins)
//         .add_plugins(WorldInspectorPlugin::new())
//         .add_plugins(SerializationPlugin)
//         .add_systems(Startup, setup)
//         .register_type::<FavoriteNumber>()
//         .register_type::<ModelVariant>()
//         //.register_type::<Option<Entity>>()
//         .run();
// }

// /// set up a simple 3D scene
// fn setup(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     commands.spawn(
//     PbrBundle {
//         mesh: meshes.add(shape::Plane::from_size(5.0).into()),
//         material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
//         ..default()
//     });
//     // mark cube to spawn
//     // commands.spawn(
//     // (

//     //     Save,
//     //     FavoriteNumber{favorite_number: 10},
//     //     ModelVariant::Cube,
//     // ));
//     // // cube
//     // commands.spawn(PbrBundle {
//     //     mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
//     //     material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
//     //     transform: Transform::from_xyz(0.0, 0.5, 0.0),
//     //     ..default()
//     // });
//     // light
//     commands.spawn(PointLightBundle {
//         point_light: PointLight {
//             intensity: 1500.0,
//             shadows_enabled: true,
//             ..default()
//         },
//         transform: Transform::from_xyz(4.0, 8.0, 4.0),
//         ..default()
//     });
//     // camera
//     commands.spawn(Camera3dBundle {
//         transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
//         ..default()
//     });
// }

// pub fn check_for_save_keypress(
//     keys: Res<Input<KeyCode>>,
// ) -> bool{
//     if keys.just_pressed(KeyCode::AltRight) {
//         return true
//     } else {
//         return false
//     }
// }

// pub fn check_for_load_keypress(
//     keys: Res<Input<KeyCode>>,
// ) -> bool{
//     if keys.just_pressed(KeyCode::AltLeft) {
//         return true
//     } else {
//         return false
//     }
// }

// pub fn spawn_unspawned_models(
//     unspawned_models_querry: Query<(Entity, &ModelVariant), Without<Handle<Mesh>>>,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
//     mut commands: Commands,
// ) {
//     for (unspawned_model, variant) in unspawned_models_querry.iter() {
//         match *variant {
//             ModelVariant::Cube => {
//                 commands.entity(unspawned_model).insert(

//                         PbrBundle {
//                                 mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
//                                 material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
//                                 transform: Transform::from_xyz(0.0, 0.5, 0.0),
//                                 ..default()
//                             }
//                 );

//             }
//         }
//     }
// }

// // pub fn list_components(
// //     model_querry: Query<(Entity), With<Handle<Mesh>>>,
// // ){
// //     for model in model_querry.iter() {
// //         model.serializable()
// //     }
// // }


// // pub fn save_into_file(
// //     path: impl Into<PathBuf>,

// // ) -> SavePipeline  {
// //     println!("saving scene");
// //     save::<With<Save>>
// //     .pipe(into_file(path.into()))
// //     .pipe(finish)
// //     .in_set(SaveSet::Save)
    
// // }