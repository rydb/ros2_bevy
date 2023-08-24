
use std::f32::consts::PI;
use crate::RaycastSource;

use crate::RaycastMethod;
use crate::DefaultRaycastingPlugin;
use crate::body::robot::components::Selectable;
use bevy::pbr::wireframe::Wireframe;
use bevy_window::PrimaryWindow;
use crate::body::robot::components::Selected;
use bevy_mod_raycast::RaycastPluginState;
use bevy::{prelude::*, reflect::TypePath, input::keyboard::KeyboardInput};
use bevy_rapier3d::prelude::{RigidBody, GravityScale};
//use body::robot::{FeatureTestPlugin, RobotTestPlugin};
use bevy_flycam::prelude::*;

use bevy::reflect::TypeUuid;

use super::components::*;

// Update our `RaycastSource` with the current cursor position every frame.
pub fn update_raycast_with_cursor(
    mut query: Query<&mut RaycastSource<Selectable>>,
    q_windows: Query<&Window, With<PrimaryWindow>>

) {
    // Grab the most recent cursor event if it exists:
    for mut pick_source in &mut query.iter_mut() {
        if let Some(cursor_pos) = q_windows.single().cursor_position() {
            pick_source.cast_method =
                bevy_mod_raycast::RaycastMethod::Screenspace(cursor_pos);
        }
    }
}

/// editor for selected rigid bodies
pub fn rigid_body_editor(
    mut commands: Commands,
    mut selected_models: Query<(Entity, &RigidBody, &Selected, &mut Transform), Without<Widget>>,
    mut materials: ResMut<Assets<StandardMaterial>>,

    keys: Res<Input<KeyCode>>,
) {
    // translation to be added after collecting all pressed key translation additions
    // some of these are definatly wrong and will need tweaking...

    // if reset rotation key is pressed, this should reset rotation to zero when set to true.
    let mut reset_rotation = false;

    // if this is enabled, model will be deselected during seelction checks for models.
    let mut deselect = false;
    //vertical/horizontal rotations
    let mut trans_to_add = Transform::from_xyz(0.0, 0.0, 0.0);
    

    if keys.pressed(KeyCode::Space) {
        trans_to_add.translation += Vec3::new(0.0, 0.1, 0.0)
    }
    if keys.pressed(KeyCode::ShiftLeft) {
        trans_to_add.translation += Vec3::new(0.0, -0.1, 0.0)
    }
    // if keys.just_pressed(KeyCode::AltLeft) {
    //     for (e, rigidbody, ..) in selected_models.iter_mut(){
    //         println!("pausing model in place");
    //         match *rigidbody {
    //             RigidBody::Dynamic => commands.entity(e).insert(RigidBody::Fixed),
    //             RigidBody::Fixed => commands.entity(e).insert(RigidBody::Dynamic),
    //             _ => todo!("other RigidBodyies besides dynamic/fixed not implemented. ")
    //         };
    //         // commands.entity(e)
    //         // .insert(RigidBody::Fixed)
    //         // ;
    //     }
    // }
    // if keys.pressed(KeyCode::Left) {
    //     trans_to_add.translation += Vec3::new(0.1, 0.0, 0.0)
    // }
    // if keys.pressed(KeyCode::Right) {
    //     trans_to_add.translation += Vec3::new(-0.1, 0.0, 0.0)
    // }
    // if keys.pressed(KeyCode::Up) {
    //     trans_to_add.translation += Vec3::new(0.0, 0.0, 0.1)
    // }
    // if keys.pressed(KeyCode::Down) {
    //     trans_to_add.translation += Vec3::new(-0.0, 0.0, -0.1)
    // }
    if keys.pressed( KeyCode::Numpad4) {
        trans_to_add.rotate(Quat::from_rotation_y(0.1))
    }
    if keys.pressed( KeyCode::Numpad6) {
        trans_to_add.rotate(Quat::from_rotation_y(-0.1))
    }
    if keys.pressed( KeyCode::Numpad8) {
        trans_to_add.rotate(Quat::from_rotation_z(-0.1))
    }
    if keys.pressed( KeyCode::Numpad2) {
        trans_to_add.rotate(Quat::from_rotation_z(0.1))
    }
    // diagonal rotations
    if keys.pressed( KeyCode::Numpad7) {
        trans_to_add.rotate(Quat::from_axis_angle(Vec3::new(1.0, 1.0, 0.0), -0.1))
    }
    if keys.pressed( KeyCode::Numpad9) {
        trans_to_add.rotate(Quat::from_axis_angle( Vec3::new(1.0, 1.0, 0.0),-0.1))
    }
    if keys.pressed( KeyCode::Numpad1) {
        trans_to_add.rotate(Quat::from_axis_angle( Vec3::new(-1.0, 1.0, 0.0),0.1))
    }
    if keys.pressed( KeyCode::Numpad3) {
        trans_to_add.rotate(Quat::from_axis_angle( Vec3::new(-1.0, 1.0, 0.0),-0.1))
    }
    if keys.pressed(KeyCode::ControlLeft) {
        reset_rotation = true;
    }

    for (e, rigidbody, selected, mut trans) in selected_models.iter_mut() {
        trans.translation += trans_to_add.translation;
        trans.rotate(trans_to_add.rotation);
        if reset_rotation == true {
            trans.rotation = Quat::IDENTITY;
        }
    }
}

/// take things that have been selected, and draw a gizmo over them to represent that
// pub fn visualize_selected_things(
//     mut gizmos: Gizmos,
//     selected_things_querry: Query<(&GlobalTransform, &Handle<Mesh>), With<Selected>>,
//     meshes: Res<Assets<Mesh>>,
// ) {
//     // padding to make wireframe not hug meshes too tightly
//     let wireframe_padding = 0.01;
//     for (trans, mesh_handle) in selected_things_querry.iter() {
//         if let Some(mesh) = meshes.get(mesh_handle) {
//             //println!("got mesh from mesh handle");
//             // get bounding box for cuboid wirefram ebased on mesh "aabb"
//             if let Some(aabb) = mesh.compute_aabb() {
//                 println!("mesh aabb: {:#?}", aabb);
//                 gizmos.cuboid(
//                     Transform::from_translation(trans.translation()).with_scale(Vec3::new(
//                         (aabb.half_extents.x * 2.0) + wireframe_padding,
//                         (aabb.half_extents.y * 2.0) + wireframe_padding,
//                         (aabb.half_extents.y * 2.0) + wireframe_padding,
//                     )
//                     ),
//                     Color::GREEN,
//                 );
//             } else {
//                 println!("unable to compute mesh aabb")
//             }
//         }
//     }

// }

/// checks for selectable things, and then selects/deselects them on various criteria
pub fn manage_selection_behaviour(    
    raycast_sources: Query<&RaycastSource<Selectable>>,
    buttons: Res<Input<MouseButton>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    valid_meshes: Query<(&Transform, &Handle<StandardMaterial>)>,
    selected_meshes: Query<&Selected>,
    mut commands: Commands,
    widget_querry: Query<(Entity), With<Widget>>,

) {
    //println!("number of raycast sources is {:#?}", raycast_sources.iter().len());
    if buttons.just_pressed(MouseButton::Left) {
        // pick nearest rigid body that camera with selector ray picks.
        for (e, intersection) in raycast_sources.iter().flat_map(|m| m.get_nearest_intersection()) {
            //println!("clicked on {:#?}, at {:#?}", e, intersection.position());
            
            if let Ok((trans, material)) = valid_meshes.get(e) {
                // attempt to fetch color from model
                if let Some(material_properties) = materials.get_mut(material) {
                    // use model ligting on and off as stand in for being selected.
                    if let Ok(..) = selected_meshes.get(e){
                        //material_properties.unlit = false;
                        
                        println!("turning off build mode");
                        commands.entity(e)
                        .remove::<Selected>()
                        .remove::<Wireframe>()
                        // if let Ok(rigidbody) = rigidbody_querry.get(e){

                        // }

                        .insert(RigidBody::Dynamic)                        ;


                        

                    } else {
                        // check if selected thing is a widget, if it is, deselect all other widgets.
                        if let Ok(_) = widget_querry.get(e) {
                            for widget in widget_querry.iter() {
                                commands.entity(widget)
                                .remove::<Selected>()
                                .remove::<Wireframe>();
                                
                            }
                        }
                        //material_properties.unlit = true;

                        commands.entity(e)
                        .insert(Selected)
                        .insert(Wireframe)
                        .insert(RigidBody::Fixed)

                        // spawn collisionless sphere thing that conveys build direction?
                        ;


                    }
                }
                else {
                    println!("failed to fetch standard material from handle, not selecting mesh for stability sake.")
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
    commands.insert_resource(RaycastPluginState::<Selectable>::default().with_debug_cursor());
    commands.spawn(
Camera3dBundle {
            transform: Transform::from_xyz(0.0, 4.0, 20.0).with_rotation(Quat::from_rotation_z(PI / 2.0)),
            ..default()
        }
        
    )
    .insert(FlyCam)
    .insert(RaycastSource::<Selectable>::new())
    ;
}

