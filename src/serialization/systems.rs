use bevy::prelude::*;

use super::components::ModelFlag;
// take unspawned models, and spawn them from flags. 
pub fn spawn_models(
    unspawned_models_query: Query<(Entity, &ModelFlag), Without<Handle<Mesh>>>,
    commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (e, model) in unspawned_models_query.iter() {
        let model_mesh_handle = meshes.add(model.geometry.clone().into());
        // commands.spawn(
        //     PbrBundle {
        //         mesh
        //     }
        // )
    }
}

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