use bevy::prelude::*;
use std::f32::consts::PI;
use bevy::reflect::TypeUuid;
use crate::RaycastSource;
use crate::RaycastMethod;
use crate::body::robot::components::Selected;
use crate::body::robot::components::MakeSelectableBundle;


/// marks that entity is widget. Used to prevent spawning widgets ontop of widgets.
#[derive(Component)]
pub struct Widget;

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
                    transform: Transform::from_translation(trans.translation + Vec3::new(0.0,dist,0.0)),
                    ..default()
                },
                MakeSelectableBundle::default(),
                Widget,
            )
        ).id();
        let y_tug_negative = commands.spawn(
            (
                PbrBundle {
                    mesh: cube_mesh.clone(),
                    material: materials.add(Color::GREEN.into()),
                    transform: Transform::from_translation(trans.translation + Vec3::new(0.0,-dist,0.0)),
                    ..default()
                },
                MakeSelectableBundle::default(),
                Widget,
            )
        ).id();
        let x_tug = commands.spawn(
            (
                PbrBundle {
                    mesh: cube_mesh.clone(),
                    material: materials.add(Color::RED.into()),
                    transform: Transform::from_translation(trans.translation + Vec3::new(dist,0.0,0.0)),
                    ..default()
                },
                MakeSelectableBundle::default(),
                Widget,
            )
        ).id();
        let x_tug_negative = commands.spawn(
        (
            PbrBundle {
                mesh: cube_mesh.clone(),
                material: materials.add(Color::RED.into()),
                transform: Transform::from_translation(trans.translation + Vec3::new(-dist,0.0,0.0)),
                ..default()
            },
            MakeSelectableBundle::default(),
            Widget,
        )
        ).id();
        let z_tug = commands.spawn(
            (
            PbrBundle {
                mesh: cube_mesh.clone(),
                material: materials.add(Color::BLUE.into()),
                transform: Transform::from_translation(trans.translation + Vec3::new(0.0,0.0,dist)),
                ..default()
            },
            MakeSelectableBundle::default(),
            Widget,
        )
        ).id();
        let z_tug_negative = commands.spawn(
            (
            PbrBundle {
                mesh: cube_mesh.clone(),
                material: materials.add(Color::BLUE.into()),
                transform: Transform::from_translation(trans.translation + Vec3::new(0.0,0.0,-dist)),
                ..default()
            },
            MakeSelectableBundle::default(),
            Widget,
        )
        ).id();
        // discs
    
        // side ring
        let y_axis_ring = commands.spawn(
            (
            PbrBundle {
                mesh: disc_mesh.clone(),
                material: materials.add(Color::BLUE.into()),
                transform: Transform::from_translation(trans.translation + Vec3::new(0.0,0.0,0.0)),
                ..default()
            },
            MakeSelectableBundle::default(),
            Widget,
        )
        ).id();
        // top ring
        let z_axis_ring = commands.spawn(
            (
            PbrBundle {
                mesh: disc_mesh.clone(),
                material: materials.add(Color::BLUE.into()),
                transform: Transform::from_translation(trans.translation + Vec3::new(0.0,0.0,0.0)).with_rotation(Quat::from_rotation_x(PI / 2.0)),
                ..default()
            },
            MakeSelectableBundle::default(),
            Widget,
        )
        ).id();


        let transform_widget = commands.spawn_empty()
        .insert(SpatialBundle::default())
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




// read which transform gizmos have been interacted with, and execute their interactions.
pub fn transform_gizmo (
    // raycast_sources: Query<&RaycastSource<SelectedForWidget>>,
    // buttons: Res<Input<MouseButton>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
    // valid_meshes: Query<(&Transform, &Handle<StandardMaterial>)>,
    // selected_meshes: Query<&SelectedForEdit>,
    // mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
){

}