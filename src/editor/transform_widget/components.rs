use bevy::prelude::*;

/// flag + direction of tug, when dragged, things with tug pull their widget in this components direction.
#[derive(Component)]
pub struct tug {
    pub pull: Vec3,
}

impl tug {
    pub fn new(x: f32,y: f32 ,z: f32) -> Self {
        Self {
            pull: Vec3::new(x, y, z)
        }
    }
}

/// flag + axis of ring, when dragged, things will rotate by their widget in this component's axis 
#[derive(Component)]
pub struct ring {
    pub axis: Vec3,
}

impl ring {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            axis: Vec3::new(x, y, z)
        }
    }
}

#[derive(Component)]
pub struct x_ring_flag;

#[derive(Component)]
pub struct y_ring_flag;

#[derive(Component)]
pub struct z_ring_flag;