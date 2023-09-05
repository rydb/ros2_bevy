use bevy::prelude::*;
use glam::Vec3;
use bevy_mod_raycast::RaycastPluginState;
use crate::editor::components::Selectable;
use super::components::*;
use crate::RaycastSource;
use std::f32::consts::PI;

/// follow behind entities marked for following
pub fn follow_flagged (
    mut commands: Commands,
    to_watch_querry: Query<(&Transform), With<Followed>>,
    viewer_querry: Query<(Entity, &Transform), With<Viewer>>,
) {
    let mut point_count = 0.0;
    let mut cord_total = Vec3::new(0.0,0.0,0.0);

    // collect all entities to follow, and then set viewer marked entities to "look at" watched entities
    for trans in to_watch_querry.iter() {
        point_count += 1.0;
        cord_total += trans.translation;
    }
    for (e, trans) in viewer_querry.iter() {
        let mut new_trans = Transform::from_translation(trans.translation);
        // look at the median cordinate between all "watched" entities
        new_trans = Transform::from_translation(cord_total + Vec3::new(0.0, 5.0, 0.0));
        //new_trans.look_at(cord_total / Vec3::new(point_count, point_count, point_count), Vec3::new(0.0,0.0,0.0));
        commands.entity(e).insert(new_trans);
    }
}

/// rotates camera to watch entities marked for watching
pub fn watch_flagged(
    mut commands: Commands,
    to_watch_querry: Query<(&Transform), With<Watched>>,
    viewer_querry: Query<(Entity, &Transform), With<Viewer>>,
) {
    let mut point_count = 0.0;
    let mut cord_total = Vec3::new(0.0,0.0,0.0);

    // collect all entities to follow, and then set viewer marked entities to "look at" watched entities
    for trans in to_watch_querry.iter() {
        point_count += 1.0;
        cord_total += trans.translation;
    }
    for (e, trans) in viewer_querry.iter() {
        let mut new_trans = *trans;
        // look at the median cordinate between all "watched" entities
        new_trans.look_at(cord_total / Vec3::new(point_count, point_count, point_count), Vec3::new(0.0,0.0,0.0));

        //println!("looking at {:#?}", new_trans.rotation);
        commands.entity(e).insert(new_trans);
    }
}

pub fn spawn_debug_cam(mut commands:Commands) {
    commands.insert_resource(RaycastPluginState::<Selectable>::default().with_debug_cursor());
    commands.spawn(
        (
Camera3dBundle {
            transform: Transform::from_xyz(20.0, 4.0, 20.0).with_rotation(Quat::from_rotation_y(PI / 2.5)),
            ..default()
        },
        //FlyCam,
        RaycastSource::<Selectable>::new(),
        SelectionMode::default(),
        //Viewer,

    )
    )
    ;
}