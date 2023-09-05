use bevy::prelude::{Component, Reflect, ReflectComponent};

/// marks entity to be looked at by entities marked with `Viewer`
#[derive(Component)]
pub struct Watched;

/// marks entity to listen to camera related system(following things, looking at things, etc..)
#[derive(Component)]
pub struct Viewer;

/// marks entity to be followed by viewers
#[derive(Component)]
pub struct Followed;

/// defines the selection mode for raycasting source: E.G: selecting would mean the camera is selecting meshes, 
/// clicking would fire a function when clicking, etc...
#[derive(Component, Clone, Copy, Reflect, Default)]
#[reflect(Component)]
pub enum SelectionMode {
    #[default]
    Selecting,
    Clicking,
}