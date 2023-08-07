use bevy::prelude::*;

// these components each have a system that listens for their existence, and then
#[derive(Component)]
pub struct x_tug_flag;

#[derive(Component)]
pub struct y_tug_flag;

#[derive(Component)]
pub struct z_tug_flag;

#[derive(Component)]
pub struct x_ring_flag;

#[derive(Component)]
pub struct y_ring_flag;

#[derive(Component)]
pub struct z_ring_flag;