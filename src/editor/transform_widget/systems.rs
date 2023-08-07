use bevy::prelude::*;
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

// Interaction check for widget.
/// Registers component for widget related events.
// #[derive(Component, Reflect, TypeUuid)]
// #[uuid = "9e31f3e9-34e2-4e47-b113-606a4b91af58"]
// pub struct SelectedForWidget{}

// /// adds raycast to mouse to click on gizmos
// pub fn add_gizmo_raycast (
//     mut cursor: EventReader<CursorMoved>,
//     mut query: Query<&mut RaycastSource<SelectedForWidget>>,

// ) {
//     // Grab the most recent cursor event if it exists:
//     let Some(cursor_moved) = cursor.iter().last() else { return };
//     for mut pick_source in &mut query {
//         pick_source.cast_method = RaycastMethod::Screenspace(cursor_moved.position);
        

//         println!("hovering over gizmos!")
        
//     }
// }
/// Manage the existence of transform widgets. Spawn transform widgets on selected models, and despawn transform widgets on unseleted models.
pub fn transform_widget_existence (
    models_without_widget: Query<(Entity, &Transform, &Selected), (Without<Widget>, Without<TransformWidgetMarker>)>,
    widgets_to_despawn: Query<(Entity, &TransformWidgetMarker), Without<Selected>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,


) {
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
    for (e, widget_marker) in widgets_to_despawn.iter() {
        commands.entity(widget_marker.transform_widget_entity)
        .despawn_recursive();
        commands.entity(e).remove::<TransformWidgetMarker>();
    }
}

//find selected y tugs, and move them to match raycast y pos for mouse raycast
pub fn manage_y_tugs(
    mut commands: Commands,
    raycast_sources: Query<&RaycastSource<Selectable>>,
    mut y_tugs: Query<(Entity, &mut Transform, ), (With<Selected>, With<y_tug_flag>)>,
    lastmouse_interactions: Query<&LastMouseInteraction>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    buttons: Res<Input<MouseButton>>,
    time: Res<Time>,


) {
    for raycastsource in raycast_sources.iter() {
        if let Some(ray) = raycastsource.ray {
            //println!("raycast origin is {:#?}", ray);
            for (e, mut tug) in y_tugs.iter_mut() {
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

                    // println!("Cursor is inside the primary window, at {:?}", position);
                    // println!("ray is origin is: {}", ray.origin() );
                    // println!("ray to_transform is: {}", ray.to_transform());
                // let vec1 = tug.translation;
                // let vec2 = ray.origin();
                // // vec project onto tub, try other way around.
                // // let vector_projection = (
                // //     (vec1 * vec2) 
                // //     /
                // //     (vec2.length() * vec2.length())
                // // ) * vec2;
                // let vector_projection = (
                //     (vec2 * vec1)
                //     /
                //     (vec1.length() * vec1.length())
                // ) * vec1;
                // println!("raycast origin is: {}", ray.origin());
                // println!("y tug origin is: {}", tug.translation);
                // println!("projecting tug to x: {}", vector_projection);
                // let mouse_delta = widget.last_mos_pos - mouse_pos;
                //println!("mouse delta is {}", mouse_delta);
                if (buttons.pressed(MouseButton::Left) && last_mouse_interaction.time_of_interaction > 0.0){
                    tug.translation.y += mouse_delta.y / 20.0; //* 2.0;

                }

                // register this mouse interaction as the last one thats happened.
                commands.entity(e).insert((mouse_inteaction));
                } 


            }

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