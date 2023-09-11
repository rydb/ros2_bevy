use bevy::prelude::*;

/// states that thing is already visualized, so no more tugs are needed.
#[derive(Component)]
pub struct Visualized;