
use std::f32::consts::PI;
use crate::RaycastSource;
use crate::RaycastMethod;
use crate::DefaultPluginState;
use bevy::{prelude::*, reflect::TypePath, input::keyboard::KeyboardInput};
use bevy_rapier3d::prelude::{RigidBody, GravityScale};
//use body::robot::{FeatureTestPlugin, RobotTestPlugin};
use bevy_flycam::prelude::*;

use bevy::reflect::TypeUuid;

// Update our `RaycastSource` with the current cursor position every frame.
pub fn update_raycast_with_cursor(
    mut cursor: EventReader<CursorMoved>,
    mut query: Query<&mut RaycastSource<RigidBody>>,

) {
    // Grab the most recent cursor event if it exists:
    let Some(cursor_moved) = cursor.iter().last() else { return };
    for mut pick_source in &mut query {
        pick_source.cast_method = RaycastMethod::Screenspace(cursor_moved.position);
        

 
        
    }
}

/// weather component is selected to be movable by build tool
#[derive(Component, Reflect, TypeUuid)]
#[uuid = "52ad446b-c48e-42a1-884f-7a0e0b74081e"]

pub struct SelectedForEdit;

/// editor for selected rigid bodies
pub fn rigid_body_editor(
    mut commands: Commands,
    mut selected_models: Query<(Entity, &RigidBody, &SelectedForEdit, &mut Transform)>,
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
    if keys.just_pressed(KeyCode::AltLeft) {
        for (e, rigidbody, ..) in selected_models.iter_mut(){
            println!("pausing model in place");
            match *rigidbody {
                RigidBody::Dynamic => commands.entity(e).insert(RigidBody::Fixed),
                RigidBody::Fixed => commands.entity(e).insert(RigidBody::Dynamic),
                _ => todo!("other RigidBodyies besides dynamic/fixed not implemented. ")
            };
            // commands.entity(e)
            // .insert(RigidBody::Fixed)
            // ;
        }
    }
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

pub fn select_rigid_body(    
    selected_meshes: Query<&RaycastSource<RigidBody>>,
    buttons: Res<Input<MouseButton>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    material_query: Query<&Handle<StandardMaterial>>,
    mut commands: Commands,
) {
    if buttons.just_pressed(MouseButton::Left) {
        for (e, intersection) in selected_meshes.iter().flat_map(|m| m.get_nearest_intersection()) {
            println!("clicked on {:#?}, at {:#?}", e, intersection.position());
            if let Ok(clicked_model) = material_query.get_component::<Handle<StandardMaterial>>(e) {
                // attempt to fetch color from model
                if let Some(material_properties) = materials.get_mut(clicked_model) {
                    // use model ligting on and off as stand in for being selected.
                    if material_properties.unlit == false{
                        material_properties.unlit = true;

                        commands.entity(e).insert(SelectedForEdit)
                        //.insert(RigidBody::Fixed);
                        // spawn collisionless sphere thing that conveys build direction?
                        ;
                    } else if material_properties.unlit == true {
                        material_properties.unlit = false;
                        println!("turning off build mode");
                        commands.entity(e).remove::<SelectedForEdit>()
                        //.insert(RigidBody::Dynamic)
                        ;
                    }
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
            transform: Transform::from_xyz(0.0, 4.0, 20.0).with_rotation(Quat::from_rotation_z(PI / 2.0)),
            ..default()
        }
        
    )
    .insert(FlyCam)
    .insert(RaycastSource::<RigidBody>::new())
    ;
}

