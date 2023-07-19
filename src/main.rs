mod body;
mod timers;
mod mesh;
mod cameras;

use bevy::{prelude::*, reflect::TypePath};
use bevy_rapier3d::prelude::RigidBody;
use body::robot::{FeatureTestPlugin, RobotTestPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_flycam::prelude::*;
use bevy_mod_raycast::{
    print_intersections, DefaultPluginState, DefaultRaycastingPlugin, RaycastMesh, RaycastMethod,
    RaycastSource, RaycastSystem,
};
//use crate::body::cube::components::*;

fn main() {
    App::new()
        .add_plugins(
            (
                DefaultPlugins,//< --- bevy needs these in order to run
                WorldInspectorPlugin::new(), // menu that displays active entities
                FeatureTestPlugin, // plugin which contains(mostly) everything program needs to run.
                NoCameraPlayerPlugin, // <-- Camera
                DefaultRaycastingPlugin::<RigidBody>::default(),

            )
        )
        .add_systems(
            First,
            update_raycast_with_cursor.before(RaycastSystem::BuildRays::<RigidBody>),
        )
        .add_systems(Startup, (spawn_debug_cam))
        .add_systems(Update, print_clicked_body)
        .run();
}

// Update our `RaycastSource` with the current cursor position every frame.
fn update_raycast_with_cursor(
    mut cursor: EventReader<CursorMoved>,
    mut query: Query<&mut RaycastSource<RigidBody>>,

) {
    // Grab the most recent cursor event if it exists:
    let Some(cursor_moved) = cursor.iter().last() else { return };
    for mut pick_source in &mut query {
        pick_source.cast_method = RaycastMethod::Screenspace(cursor_moved.position);
        

 
        
    }
}

fn print_clicked_body(    
    selected_meshes: Query<&RaycastSource<RigidBody>>,
    buttons: Res<Input<MouseButton>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    material_query: Query<&Handle<StandardMaterial>>,
    mut commands: Commands,
) {
    if buttons.just_pressed(MouseButton::Left) {
        for (e, intersection) in selected_meshes.iter().flat_map(|m| m.intersections()) {
            println!("clicked on {:#?}, at {:#?}", e, intersection.position());
            if let Ok(clicked_model) = material_query.get_component::<Handle<StandardMaterial>>(*e) {
                // attempt to fetch color from model
                if let Some(model_color) = materials.get_mut(clicked_model) {
                    model_color.base_color = Color::GREEN
                }
                else {
                    println!("failed to fetch standard material from handle")
                }
            }
            else {
                println!("failed to fetch handle to standard material for model");
                //println!("model components are {:#?}", commands.entity(*e).log_components())
            }
        }
    }
}
// pub fn spawn_raycast_from_camera(mut comands: Commands) {

// }

///spawns camera for debug
pub fn spawn_debug_cam(mut commands:Commands) {
    commands.insert_resource(DefaultPluginState::<RigidBody>::default().with_debug_cursor());
    commands.spawn(
Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        }
        
    )
    .insert(FlyCam)
    .insert(RaycastSource::<RigidBody>::new())
    ;
}

