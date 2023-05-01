mod body;


use bevy::prelude::*;
use bevy_flycam::PlayerPlugin;
use bevy_rapier3d::prelude::*;
use bevy_obj::*;

//use crate::body::cube::components::*;
use crate::body::robot::components::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(PlayerPlugin)
        .add_plugin(RobotPlugin)
        //.add_plugin(CubePlugin)
        .add_plugin(ObjPlugin) // for loading obj meshes
        //.add_startup_system(setup_graphics)
        .add_startup_system(setup_physics)
        .run();
}

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(100.0, 0.1, 100.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)));

}