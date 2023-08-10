use std::default;
use bevy::reflect::TypeUuid;
/// marks that entity is widget. Used to prevent spawning widgets ontop of widgets.
use bevy::prelude::*;

#[derive(Component)]
pub struct Widget;

/// A component which stores the last mouse interaction that happened to an entity. Something that wants mouse functionality should use/over-write this.
#[derive(Component, Clone, Copy, Reflect)]
#[reflect(Component)]
pub struct LastMouseInteraction {
    pub mouse_pos: Vec2,

    pub time_of_interaction: f64,
    //pub hold_duration: Option<f32>,
}

impl Default for LastMouseInteraction {
    fn default() -> Self {
        Self {
            mouse_pos: Vec2::default(),
            time_of_interaction: 0.0,
            //hold_duration: None,
        }
    }
}
