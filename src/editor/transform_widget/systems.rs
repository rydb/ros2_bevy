use bevy::ecs::query::QueryParIter;
use bevy::prelude::*;
use bevy::transform;
use std::f32::consts::PI;
use bevy::reflect::TypeUuid;
use crate::RaycastSource;
use crate::body::robot::components::Selected;
use crate::body::robot::components::MakeSelectableBundle;
use crate::editor::components::*;
use super::components::*;
use crate::body::robot::components::Selectable;

use bevy_window::PrimaryWindow;
use bevy::input::mouse::MouseMotion;
/// marker that states: WHICH transform widget entity has its transform based on. 
#[derive(Component)]
pub struct TransformWidgetMarker {
    transform_widget_entity: Entity,
    /// entity to be modified by transform widget
    entity_to_transform: Entity, 
}


// despawn transform widgets around things that have been de selected
pub fn widget_despawn_for_deselected(
    widgets_to_despawn: Query<(Entity, &TransformWidgetMarker), Without<Selected>>,
    mut commands: Commands,
) {
    for (e, widget_marker) in widgets_to_despawn.iter() {
        commands.entity(widget_marker.transform_widget_entity)
        .despawn_recursive();
        commands.entity(e).remove::<TransformWidgetMarker>();
    }
}

/// spawn widgets around things that have been selected
pub fn widget_spawn_for_selected (
    models_without_widget: Query<(Entity, &Transform, &Selected), (Without<Widget>, Without<TransformWidgetMarker>)>,
    widgets_to_despawn: Query<(Entity, &TransformWidgetMarker), Without<Selected>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,


) {
    //println!("transform widget existence called");
    //spawn transform widgets on selected entities
    for (e, trans,..) in models_without_widget.iter() {
        

        

        let cube_size = 0.3;

        let dist = 1.0;
    
        let cube_mesh = meshes.add(shape::Cube{size: cube_size}.into());
    
        let disc_mesh = meshes.add(shape::Torus{
            radius: dist,
            ring_radius: 0.1,
            subdivisions_segments: 10,
            subdivisions_sides: 10,
        }.into());
    
        // spawn edit widget, x = red, y = green, z = blue
        
        // some these are probably wrong and will need tweaking...
        let y_tug = commands.spawn(
        (
                PbrBundle {
                    mesh: cube_mesh.clone(),
                    material: materials.add(Color::GREEN.into()),
                    transform: Transform::from_translation(Vec3::new(0.0,dist,0.0)),
                    ..default()
                },
                MakeSelectableBundle::default(),
                Widget,
                y_tug_flag,
            )
        ).id();
        let y_tug_negative = commands.spawn(
            (
                PbrBundle {
                    mesh: cube_mesh.clone(),
                    material: materials.add(Color::GREEN.into()),
                    transform: Transform::from_translation(Vec3::new(0.0,-dist,0.0)),
                    ..default()
                },
                MakeSelectableBundle::default(),
                Widget,
                y_tug_flag,
            )
        ).id();
        let x_tug = commands.spawn(
            (
                PbrBundle {
                    mesh: cube_mesh.clone(),
                    material: materials.add(Color::RED.into()),
                    transform: Transform::from_translation(Vec3::new(dist,0.0,0.0)),
                    ..default()
                },
                MakeSelectableBundle::default(),
                Widget,
                x_tug_flag,
            )
        ).id();
        let x_tug_negative = commands.spawn(
        (
            PbrBundle {
                mesh: cube_mesh.clone(),
                material: materials.add(Color::RED.into()),
                transform: Transform::from_translation(Vec3::new(-dist,0.0,0.0)),
                ..default()
            },
            MakeSelectableBundle::default(),
            Widget,
            x_tug_flag,
        )
        ).id();
        let z_tug = commands.spawn(
            (
            PbrBundle {
                mesh: cube_mesh.clone(),
                material: materials.add(Color::BLUE.into()),
                transform: Transform::from_translation(Vec3::new(0.0,0.0,dist)),
                ..default()
            },
            MakeSelectableBundle::default(),
            Widget,
            z_tug_flag,
        )
        ).id();
        let z_tug_negative = commands.spawn(
            (
            PbrBundle {
                mesh: cube_mesh.clone(),
                material: materials.add(Color::BLUE.into()),
                transform: Transform::from_translation(Vec3::new(0.0,0.0,-dist)),
                ..default()
            },
            MakeSelectableBundle::default(),
            Widget,
            z_tug_flag,
        )
        ).id();
        // discs
    
        // side ring
        let y_axis_ring = commands.spawn(
            (
            PbrBundle {
                mesh: disc_mesh.clone(),
                material: materials.add(Color::BLUE.into()),
                transform: Transform::from_translation(Vec3::new(0.0,0.0,0.0)),
                ..default()
            },
            MakeSelectableBundle::default(),
            Widget,
            y_ring_flag,
        )
        ).id();
        // top ring
        let z_axis_ring = commands.spawn(
            (
            PbrBundle {
                mesh: disc_mesh.clone(),
                material: materials.add(Color::BLUE.into()),
                transform: Transform::from_translation(Vec3::new(0.0,0.0,0.0)).with_rotation(Quat::from_rotation_x(PI / 2.0)),
                ..default()
            },
            MakeSelectableBundle::default(),
            Widget,
            z_ring_flag,
        )
        ).id();


        let transform_widget = commands.spawn_empty()
        // set widget root transform to equal model the widget is spawning around
        .insert(SpatialBundle::from_transform(Transform::from_translation(trans.translation)))
        .add_child(y_tug)
        .add_child(y_tug_negative)
        .add_child(x_tug)
        .add_child(x_tug_negative)
        .add_child(z_tug)
        .add_child(z_tug_negative)
        .add_child(y_axis_ring)
        .add_child(z_axis_ring)
        .id()
        ;

        commands.entity(e)
        .insert(
            TransformWidgetMarker {
                transform_widget_entity: transform_widget,
                entity_to_transform: e,

            }
        );

        // commands.entity(e)
        // .add_child(transform_widget);
    }
    //despawn transform widgets on deselected entites.

}

//find selected y tugs, and move them to match raycast y pos for mouse raycast

///
/// NOTE 
/// 
/// We want to start from the parent and derive components from there. Transforms/Global Transforms shoul be gotten with .get to avoid double linked list
/// problem from occuring (parent <--> child, unsafe), (parent --> child, safe)
pub fn manage_y_tugs(
    mut commands: Commands,
    y_tugs: Query<Entity, (With<Selected>, With<y_tug_flag>)>,
    lastmouse_interactions: Query<&LastMouseInteraction>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    buttons: Res<Input<MouseButton>>,
    time: Res<Time>,
    transform_querry: Query<&Transform>,
    parent_querry: Query<&Parent>,  

) {


    for e in y_tugs.iter() {

        if let Some(mouse_pos) = q_windows.single().cursor_position() {
            let mouse_inteaction = LastMouseInteraction {
                mouse_pos: mouse_pos,
                time_of_interaction: time.delta_seconds_f64()
            };
            let mut last_mouse_interaction = LastMouseInteraction::default();
            if let Ok(mouse_check) = lastmouse_interactions.get(e) {
                last_mouse_interaction = *mouse_check
            } 
            let mouse_delta = last_mouse_interaction.mouse_pos - mouse_inteaction.mouse_pos;
    
        if buttons.pressed(MouseButton::Left) && last_mouse_interaction.time_of_interaction > 0.0 {
            //tug.translation.y += mouse_delta.y / 20.0; //* 2.0;
            if let Some(root_ancestor) = parent_querry.iter_ancestors(e).last() {
                let widget_root_transform = transform_querry.get(root_ancestor).unwrap();
    
                //println!("inserting transform for x tug at time{:#?}", time.delta());
                commands.entity(root_ancestor).insert(
                Transform::from_xyz(
                    widget_root_transform.translation.x,
                    widget_root_transform.translation.y  + mouse_delta.y / 20.0, //* 2.0;
                    widget_root_transform.translation.z,
                )
    
                );
            }
        }
    
        // register this mouse interaction as the last one thats happened.
        commands.entity(e).insert(mouse_inteaction);
        } 
    }
}

pub fn manage_x_tugs (
    mut commands: Commands,
    y_tugs: Query<Entity, (With<Selected>, With<x_tug_flag>)>,
    lastmouse_interactions: Query<&LastMouseInteraction>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    buttons: Res<Input<MouseButton>>,
    time: Res<Time>,
    transform_querry: Query<&Transform>,
    parent_querry: Query<&Parent>,  
) {

    for e in y_tugs.iter() {

        if let Some(mouse_pos) = q_windows.single().cursor_position() {
            let mouse_inteaction = LastMouseInteraction {
                mouse_pos: mouse_pos,
                time_of_interaction: time.delta_seconds_f64()
            };
            let mut last_mouse_interaction = LastMouseInteraction::default();
            if let Ok(mouse_check) = lastmouse_interactions.get(e) {
                last_mouse_interaction = *mouse_check
            } 
            let mouse_delta = last_mouse_interaction.mouse_pos - mouse_inteaction.mouse_pos;
    
        if buttons.pressed(MouseButton::Left) && last_mouse_interaction.time_of_interaction > 0.0 {
            //tug.translation.y += mouse_delta.y / 20.0; //* 2.0;
            if let Some(root_ancestor) = parent_querry.iter_ancestors(e).last() {
                let widget_root_transform = transform_querry.get(root_ancestor).unwrap();
    
                //println!("inserting transform for x tug at time{:#?}", time.delta());
                commands.entity(root_ancestor).insert(
                Transform::from_xyz(
                    widget_root_transform.translation.x + -mouse_delta.x / 20.0,
                    widget_root_transform.translation.y , //* 2.0;
                    widget_root_transform.translation.z,
                )
    
                );
            }
        }
    
        // register this mouse interaction as the last one thats happened.
        commands.entity(e).insert(mouse_inteaction);
        } 
    }
    
}

pub fn manage_z_tugs (
    mut commands: Commands,
    y_tugs: Query<Entity, (With<Selected>, With<z_tug_flag>)>,
    lastmouse_interactions: Query<&LastMouseInteraction>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    buttons: Res<Input<MouseButton>>,
    time: Res<Time>,
    transform_querry: Query<&Transform>,
    parent_querry: Query<&Parent>,  
) {

    for e in y_tugs.iter() {

        if let Some(mouse_pos) = q_windows.single().cursor_position() {
            let mouse_inteaction = LastMouseInteraction {
                mouse_pos: mouse_pos,
                time_of_interaction: time.delta_seconds_f64()
            };
            let mut last_mouse_interaction = LastMouseInteraction::default();
            if let Ok(mouse_check) = lastmouse_interactions.get(e) {
                last_mouse_interaction = *mouse_check
            } 
            let mouse_delta = last_mouse_interaction.mouse_pos - mouse_inteaction.mouse_pos;
    
        if buttons.pressed(MouseButton::Left) && last_mouse_interaction.time_of_interaction > 0.0 {
            //tug.translation.y += mouse_delta.y / 20.0; //* 2.0;
            if let Some(root_ancestor) = parent_querry.iter_ancestors(e).last() {
                let widget_root_transform = transform_querry.get(root_ancestor).unwrap();
    
                commands.entity(root_ancestor).insert(
                Transform::from_xyz(
                    widget_root_transform.translation.x,
                    widget_root_transform.translation.y , //* 2.0;
                    widget_root_transform.translation.z + -mouse_delta.y / 20.0,
                )
    
                );
            }
        }
    
        // register this mouse interaction as the last one thats happened.
        commands.entity(e).insert(mouse_inteaction);
        } 
    }
    
}

pub fn manage_y_rings(
    mut commands: Commands,
    y_tugs: Query<Entity, (With<Selected>, With<y_ring_flag>)>,
    lastmouse_interactions: Query<&LastMouseInteraction>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    buttons: Res<Input<MouseButton>>,
    time: Res<Time>,
    transform_querry: Query<&Transform>,
    parent_querry: Query<&Parent>,

) {

    for e in y_tugs.iter() {

        if let Some(mouse_pos) = q_windows.single().cursor_position() {
            let mouse_inteaction = LastMouseInteraction {
                mouse_pos: mouse_pos,
                time_of_interaction: time.delta_seconds_f64()
            };
            let mut last_mouse_interaction = LastMouseInteraction::default();
            if let Ok(mouse_check) = lastmouse_interactions.get(e) {
                last_mouse_interaction = *mouse_check
            } 
            let mouse_delta = last_mouse_interaction.mouse_pos - mouse_inteaction.mouse_pos;

        if buttons.pressed(MouseButton::Left) && last_mouse_interaction.time_of_interaction > 0.0 {
            //tug.translation.y += mouse_delta.y / 20.0; //* 2.0;
            if let Some(root_ancestor) = parent_querry.iter_ancestors(e).last() {
                let widget_root_transform = transform_querry.get(root_ancestor).unwrap();

                // take transform of widget, and rotate root widget based on that.
                let mut new_transform = *widget_root_transform;
                new_transform.rotate_y(-mouse_delta.x * 0.02); 

                commands.entity(root_ancestor).insert(new_transform);
                println!("new transform is {:#?}", new_transform)
            }
        }

        // register this mouse interaction as the last one thats happened.
        commands.entity(e).insert(mouse_inteaction);
        } 
    }     
}
pub fn manage_z_rings(
    mut commands: Commands,
    y_tugs: Query<Entity, (With<Selected>, With<z_ring_flag>)>,
    lastmouse_interactions: Query<&LastMouseInteraction>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    buttons: Res<Input<MouseButton>>,
    time: Res<Time>,
    transform_querry: Query<&Transform>,
    parent_querry: Query<&Parent>,

) {

    for e in y_tugs.iter() {

        if let Some(mouse_pos) = q_windows.single().cursor_position() {
            let mouse_inteaction = LastMouseInteraction {
                mouse_pos: mouse_pos,
                time_of_interaction: time.delta_seconds_f64()
            };
            let mut last_mouse_interaction = LastMouseInteraction::default();
            if let Ok(mouse_check) = lastmouse_interactions.get(e) {
                last_mouse_interaction = *mouse_check
            } 
            let mouse_delta = last_mouse_interaction.mouse_pos - mouse_inteaction.mouse_pos;

        if buttons.pressed(MouseButton::Left) && last_mouse_interaction.time_of_interaction > 0.0 {
            //tug.translation.y += mouse_delta.y / 20.0; //* 2.0;
            if let Some(root_ancestor) = parent_querry.iter_ancestors(e).last() {
                let widget_root_transform = transform_querry.get(root_ancestor).unwrap();

                // take transform of widget, and rotate root widget based on that.
                let mut new_transform = *widget_root_transform;
                new_transform.rotate_z(mouse_delta.y * 0.02); 

                commands.entity(root_ancestor).insert(new_transform);
                println!("new transform is {:#?}", new_transform)
            }
        }

        // register this mouse interaction as the last one thats happened.
        commands.entity(e).insert(mouse_inteaction);
        } 
    }     
}


// read which transform widgets have been interacted with, execute the behavour of the selected widgets parts.
pub fn transform_widget_behaviour (
    // raycast_sources: Query<&RaycastSource<SelectedForWidget>>,
    // buttons: Res<Input<MouseButton>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
    // valid_meshes: Query<(&Transform, &Handle<StandardMaterial>)>,
    // selected_meshes: Query<&SelectedForEdit>,
    mut commands: Commands,
    models_with_widget: Query<(Entity, &GlobalTransform, &TransformWidgetMarker)>,
    transform_querry: Query<(&Transform)>,

){
    for (e,trans, widget_marker) in models_with_widget.iter() {
    
        // set widget translation to equal model translation
        //commands.entity(widget_marker.transform_widget_entity).insert(Transform::from_translation(trans.translation()));
        // set model transform to equal gizmo transform
        if let Ok (transform_widget_transform) = transform_querry.get(widget_marker.transform_widget_entity){
            //println!("setting {:#?} to match global transform for widget at {:#?}", e, transform_widget_transform.translation);
            commands.entity(e).insert(*transform_widget_transform);
        }else {
            //println!("failed to get global transform of transform widget because ???");
        }

    
    }
}