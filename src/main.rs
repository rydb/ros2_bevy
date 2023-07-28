mod body;
mod timers;
mod mesh;
mod editor;

use bevy::{prelude::*, reflect::TypePath, input::keyboard::KeyboardInput};
use bevy_rapier3d::prelude::{RigidBody, GravityScale, ImpulseJoint};
use body::robot::{FeatureTestPlugin, RobotTestPlugin};
use bevy_flycam::prelude::*;
use bevy_mod_raycast::{
    print_intersections, DefaultPluginState, DefaultRaycastingPlugin, RaycastMesh, RaycastMethod,
    RaycastSource, RaycastSystem,
};
use editor::plugins::EditorPlugin;
use bevy_rapier3d::rapier::dynamics::JointAxis;
//use crate::body::cube::components::*;

fn main() {
    App::new()
        .add_plugins(
            (
                DefaultPlugins,//< --- bevy needs these in order to run
                FeatureTestPlugin, // plugin which contains(mostly) everything program needs to run.
                NoCameraPlayerPlugin, // <-- Camera
                EditorPlugin,
                

            )
        )
        .add_systems(Update, drive_wheels)
        .run();
}

//drive wheels forward
fn drive_wheels(
    commands: Commands,
    mut wheel_query: Query<(Entity, &mut ImpulseJoint)>,
    keys: Res<Input<KeyCode>>,

) {
        // translation to be added after collecting all pressed key translation additions
    // some of these are definatly wrong and will need tweaking...

    // if reset rotation key is pressed, this should reset rotation to zero when set to true.
    let mut reset_rotation = false;

    // if this is enabled, model will be deselected during seelction checks for models.
    let mut deselect = false;
    //vertical/horizontal rotations    
    let speed_multiplier = 10.0;
    
    let mut drive_velocity = 0.0;

    // if keys.pressed(KeyCode::Space) {
    //     direction_to_drive.translation += Vec3::new(0.0, 0.1, 0.0) * speed_multiplier
    // }
    // if keys.pressed(KeyCode::ShiftLeft) {
    //     direction_to_drive.translation += Vec3::new(0.0, -0.1, 0.0) * speed_multiplier
    // }
    // if keys.pressed(KeyCode::Left) {
    //     direction_to_drive.translation += Vec3::new(0.1, 0.0, 0.0) * speed_multiplier
    // }
    // if keys.pressed(KeyCode::Right) {
    //     direction_to_drive.translation += Vec3::new(-0.1, 0.0, 0.0) * speed_multiplier
    // }
    if keys.pressed(KeyCode::Up) {
        drive_velocity += -1.0 * speed_multiplier;
        println!("driving twoards {:#?}", drive_velocity);

    }
    if keys.pressed(KeyCode::Down) {
        drive_velocity += 1.0 * speed_multiplier;
        println!("driving twoards {:#?}", drive_velocity);

    }

    for (e, mut joint) in wheel_query.iter_mut() {
        // commands.entity(e)
        // .insert()
        joint.data.set_motor_velocity(JointAxis::AngX, drive_velocity, speed_multiplier);
    }
}