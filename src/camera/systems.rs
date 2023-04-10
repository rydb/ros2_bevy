use bevy::prelude::*;
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};



pub fn spawn_fly_camera(
    mut commands: Commands,
) {
    commands
    .spawn(
        (
            Camera3dBundle {
                transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
                ..default()
            },
            FlyCamera::default()
        
        )
    )

    ;
}

//pub fn set_fly_camera(commands: &mut Commands, camera_query) {}


// set a camera to be a fly camera by the FlyCamera Plugin
//pub fn set_fly_camera(commands: &mut Commands, camera_query) {

//}