use bevy::prelude::*;

// flag + direction of of tug, when dragged, things with tug pull their widget in their direction.
#[derive(Component)]
pub struct tug {
    pub pull: Vec3,
}

impl tug {
    pub fn new(x: f32,y: f32 ,z: f32) -> Self {
        return Self {
            pull: Vec3::new(x, y, z)
        }
    }
}

#[derive(Component)]
pub struct x_ring_flag;

#[derive(Component)]
pub struct y_ring_flag;

#[derive(Component)]
pub struct z_ring_flag;