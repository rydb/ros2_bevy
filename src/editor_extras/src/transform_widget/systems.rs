use bevy::prelude::*;
use std::f32::consts::PI;
use crate::components::*;
use super::gizmo_material::GizmoMaterial;
use bevy_mod_raycast::RaycastSource;
use bevy_window::PrimaryWindow;
/// marker that states: WHICH transform widget entity has its transform based on. 
use crate::transform_widget::components::*;
use component_extras::components::*;

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
    //widgets_to_despawn: Query<(Entity, &TransformWidgetMarker), Without<Selected>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut gizmo_material: ResMut<Assets<GizmoMaterial>>,

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
        let (s, l) = (0.8, 0.6);
        let y_tug = commands.spawn(
        (
                MaterialMeshBundle {
                    mesh: cube_mesh.clone(),
                    material: gizmo_material.add(GizmoMaterial::from(Color::hsl(120.0, s, l))),
                    transform: Transform::from_translation(Vec3::new(0.0,dist,0.0)),
                    ..default()
                },
                MakeSelectableBundle::default(),
                Widget,
                Tug::new(0.0,1.0,0.0),
            )
        ).id();
        let y_tug_negative = commands.spawn(
            (
                MaterialMeshBundle {
                    mesh: cube_mesh.clone(),
                    material: gizmo_material.add(GizmoMaterial::from(Color::hsl(120.0, s, l))),
                    transform: Transform::from_translation(Vec3::new(0.0,-dist,0.0)),
                    ..default()
                },
                MakeSelectableBundle::default(),
                Widget,
                Tug::new(0.0,1.0,0.0),
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
                Tug::new(1.0,0.0,0.0),
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
            Tug::new(1.0,0.0,0.0),
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
            Tug::new(0.0,0.0,1.0),
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
            Tug::new(0.0,0.0,1.0),
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
            //y_ring_flag,
            Ring::new(0.0, 1.0, 0.0),
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
            //z_ring_flag,
            Ring::new(0.0, 0.0, 1.0),
        )
        ).id();


        let transform_widget = commands.spawn(
            (
                SpatialBundle::from_transform(Transform::from_translation(trans.translation)),
                TransformWidget {bound_entity: e}
            )
        )
        // set widget root transform to equal model the widget is spawning around
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

// 
pub fn manage_tugs(
    mut commands: Commands,
    selected_tugs: Query<(Entity, &Tug), With<Selected>>,
    lastmouse_interactions: Query<&LastMouseInteraction>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    buttons: Res<Input<MouseButton>>,
    time: Res<Time>,
    mut transform_querry: Query<&mut Transform, Without<SelectionMode>>,
    transform_immutable_querry: Query<&Transform, With<SelectionMode>>,
    parent_querry: Query<&Parent>,  
    transform_widget_querry: Query<&TransformWidget>,
    raycast_sources: Query<(Entity, &RaycastSource<Selectable>)>,


) {
    // how much pull of tugs should be reduced
    let tug_sensitivity_divisor = 20.0;
    // made to crash on multiple cameras on purpose. this will need to be refactored when multiple cams exist.
    if let Ok(selector_cam_trans) = transform_immutable_querry.get(raycast_sources.single().0) {
        for (e, tug) in selected_tugs.iter() {

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
                    if let Ok(transform_widget_flag) = transform_widget_querry.get(root_ancestor) {
                        if let Ok(mut widget_root_transform) = transform_querry.get_mut(transform_widget_flag.bound_entity) {
                            //let widget_root_transform = *bound_model_transform;
                            let (cam_x, cam_y, cam_z) = (
                                (widget_root_transform.translation.x - selector_cam_trans.translation.x),
                                (widget_root_transform.translation.y - selector_cam_trans.translation.y),
                                (widget_root_transform.translation.z - selector_cam_trans.translation.z)); 
                            let (cam_x_rot, cam_y_rot, cam_z_rot) = selector_cam_trans.rotation.to_euler(EulerRot::XYZ);
                            
                            // math for gizmos
                            
                            widget_root_transform.translation = Vec3::new(
                                widget_root_transform.translation.x + (
                                    tug.pull.x * 
                                    (
                                        ((-mouse_delta.x * (1.0 - (cam_y_rot / (PI / 2.0)))) + ((-mouse_delta.y) * (-cam_y_rot / (PI / 2.0))))  / tug_sensitivity_divisor
                                    )) * (cam_z / cam_z.abs())  
                                    //* (cam_x / cam_x.abs()) ,
                                    ,
                                widget_root_transform.translation.y + (tug.pull.y * (mouse_delta.y / tug_sensitivity_divisor))
                                    //(cam_y / cam_y.abs()), //* 2.0;
                                    ,
                                widget_root_transform.translation.z + 
                                (tug.pull.z * 
                                    (
                                        ((-mouse_delta.y * (1.0 - (cam_y_rot / (PI / 2.0)))) + ((-mouse_delta.x) * (cam_y_rot / (PI / 2.0))))  / tug_sensitivity_divisor
                                    )
                                )
                                * (cam_x / cam_x.abs())/* (cam_y_rot / cam_y_rot.abs())*/
                                    
                            );
                            //println!("mouse interaction last {:#}", last_mouse_interaction.mouse_pos);
                            //println!("mouse interaction current {:#}", last_mouse_interaction.mouse_pos);
                            //println!("mouse delta is {:#?}", mouse_delta);
                            
                            //println!("widget rotation is now {:#?}", widget_root_transform.rotation);
                            //println!("mouse drag is {:#?}", mouse_delta);
                            println!("camera y rot is {:#?}", cam_y_rot);
                            println!("mouse delta y multipler is {:#?}", (1.0 - (cam_y_rot.abs() / (PI / 2.0))));
                            println!("mouse delta x multipler is {:#?}", (cam_y_rot.abs() / (PI / 2.0)));
                            println!("x alignment multiplier is {:#?}", cam_x / cam_x.abs());
                            println!("y alignment multiplier is {:#?}", cam_y / cam_y.abs());
                            println!("z alignment multiplier is {:#?}", cam_z / cam_z.abs());
                            //println!("mouse drag is modified to be : {:#?}", ((-mouse_delta.y * (cam_y_rot.abs() % PI)) + ((-mouse_delta.x) * (cam_y_rot.abs() / PI)))  / tug_sensitivity_divisor);
                            //println!("camera rotation y is now {:#?}", selector_cam_trans.rotation.to_euler(EulerRot::XYZ));
                            //println!("camera and widget rotation difference is {:#?}", widget_root_transform.rotation - selector_cam_trans.rotation)
                            //println!("widget translation is now {:#?}", widget_root_transform.translation);
                           // println!("camera translation is now {:#?}", selector_cam_trans.translation);
                            //println!("camera dist from widget is {:#?}", widget_root_transform.translation - selector_cam_trans.translation);
                            
                            
                            //println!("cam z orientation is {:#?}", (cam_x / cam_x.abs()));
                            //println!("cam z is {:#?}", cam_z);
                            //println!("inserting transform for x tug at time{:#?}", time.delta());
                            // commands.entity(transform_widget_flag.bound_entity).insert(
                            //     Transform::from_xyz(
                            //         widget_root_transform.translation.x + (tug.pull.x * (-mouse_delta.x / tug_sensitivity_divisor)),
                            //         widget_root_transform.translation.y + (tug.pull.y * (mouse_delta.y / tug_sensitivity_divisor)), //* 2.0;
                            //         widget_root_transform.translation.z + (tug.pull.z * (-mouse_delta.y / tug_sensitivity_divisor))
                            //     )
                    
                            //     );
                            
                        }
                    }
    
                }
            }
        
            // register this mouse interaction as the last one thats happened.
            commands.entity(e).insert(mouse_inteaction);
            } 
        }
    }

}

/// Correlate movements of selected rings into rotations into rotation of bound object. 
pub fn manage_rings(
    mut commands: Commands,
    rings: Query<(Entity, &Ring), With<Selected>>,
    lastmouse_interactions: Query<&LastMouseInteraction>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    buttons: Res<Input<MouseButton>>,
    time: Res<Time>,
    mut transform_querry: Query<&mut Transform>,
    parent_querry: Query<&Parent>,
    transform_widget_querry: Query<&TransformWidget>

) {
    // how sensitive rings are to mouse drags for rotation
    let ring_sensitivity_divisor = 20.0;
    for (e, ring) in rings.iter() {

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
                //let widget_root_transform = transform_querry.get(root_ancestor).unwrap();

                // take transform of widget, and rotate root widget based on that.
                if let Ok(transform_widget_flag) = transform_widget_querry.get(root_ancestor) {
                    if let Ok(mut ring_transform) = transform_querry.get_mut(transform_widget_flag.bound_entity) {
                        
                        //let mut new_transform = *bound_model_transform;
                        ring_transform.rotate_y(-mouse_delta.x * 0.02); 
                        
                        let mouse_delta_with_z = Vec3::new(mouse_delta.x, mouse_delta.y, mouse_delta.y);
                        // how do we make ring axis rotations add up and stil be commutive???
    
                        println!("rotating cube based on ring rotation");
                        ring_transform.rotate_axis(ring.axis, (ring.axis.dot(mouse_delta_with_z)) / ring_sensitivity_divisor);
                        //commands.entity(transform_widget_flag.bound_entity).insert(new_transform);
                        //println!("new transform is {:#?}", new_transform)
                    }

                }

            }
        }

        // register this mouse interaction as the last one thats happened.
        commands.entity(e).insert(mouse_inteaction);
        } 
    }     
}

// read which transform widgets have been interacted with, execute the behavour of the selected widgets parts.
pub fn transform_widget_behaviour (
    //mut commands: Commands,
    mut transform_widget_query: Query<(&mut Transform, &TransformWidget)>,
    mut transform_querry: Query<&mut Transform, Without<TransformWidget>>,

){
    for (mut widget_trans, transform_widget_flag) in transform_widget_query.iter_mut() {
        if let Ok(model_transform) = transform_querry.get_mut(transform_widget_flag.bound_entity) {
            //model_transform.rotation = Quat::IDENTITY;
            widget_trans.translation = model_transform.translation;
        }
    }
}