use bevy::prelude::*;
use glam::Vec3;
use bevy_mod_raycast::RaycastPluginState;
use bevy_mod_raycast::RaycastSource;
use std::f32::consts::PI;
use super::components::*;
use bevy_flycam::FlyCam;
use component_extras::components::*;

/// follow behind entities marked for following
pub fn follow_flagged (
    //mut commands: Commands,
    to_watch_querry: Query<Entity, With<Followed>>,
    viewer_querry: Query<(Entity, &Viewer)>,
    mut transform_querry: Query<&mut Transform>,
) {
    let mut cord_total = Vec3::new(0.0,0.0,0.0);

    for e in to_watch_querry.iter() {
        if let Ok(trans) = transform_querry.get(e) {
            cord_total += trans.translation;
        }
    }
    for (e, viewer) in viewer_querry.iter() {
        if let Ok(mut trans) = transform_querry.get_mut(e) {
            //println!("new trans for FOLLOW is {:#}", new_trans.translation);
            trans.translation = cord_total + viewer.offset;
            //println!("following all followed entities at: {:#?}", new_trans.translation);
            //commands.entity(e).insert(new_trans);
        }

    }
}

/// rotates camera to watch entities marked for watching
pub fn watch_flagged(
    //mut commands: Commands,
    to_watch_querry: Query<Entity, With<Watched>>,
    viewer_querry: Query<Entity, With<Viewer>>,
    mut transform_querry: Query<&mut Transform>,

) {
    let mut point_count = 0.0;
    let mut cord_total = Vec3::new(0.0,0.0,0.0);

    for e in to_watch_querry.iter() {
        if let Ok(trans) = transform_querry.get(e) {
            point_count += 1.0;
            cord_total += trans.translation;
        }
    }
    for e in viewer_querry.iter() {
        if let Ok(mut trans) = transform_querry.get_mut(e) {
            //let mut new_trans = *trans;
            //println!("new trans for ROTATION is: {:#?}", new_trans.translation);
            // look at the median cordinate between all "watched" entities
            trans.look_at(cord_total / Vec3::new(point_count, point_count, point_count), Vec3::new(0.0,0.0,0.0));
            //println!("looking at {:#?}", new_trans.rotation);
            //commands.entity(e).insert(new_trans);
        }

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
        FlyCam,
        RaycastSource::<Selectable>::new(),
        SelectionMode::default(),
        Viewer{offset: Vec3::new(5.0, 5.0, 5.0)},

    )
    )
    ;
}