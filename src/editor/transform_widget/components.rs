use bevy::prelude::*;

/// flag + direction of tug, when dragged, things with tug pull their widget in this components direction.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Tug {
    pub pull: Vec3,
}

impl Tug {
    pub fn new(x: f32,y: f32 ,z: f32) -> Self {
        Self {
            pull: Vec3::new(x, y, z)
        }
    }
}

/// flag + axis of ring, when dragged, things will rotate by their widget in this component's axis 
#[derive(Component)]
pub struct Ring {
    pub axis: Vec3,
}

impl Ring {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            axis: Vec3::new(x, y, z)
        }
    }
}

// Collects commands from widgets, applies them to bound widget command reciever
#[derive(Component)]
pub struct TransformWidget {
    pub bound_entity: Entity,
}

#[derive(Component)]
pub struct TransformWidgetMarker {
    pub transform_widget_entity: Entity,
    /// entity to be modified by transform widget
    pub entity_to_transform: Entity, 
}


