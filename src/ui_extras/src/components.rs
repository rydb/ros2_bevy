use std::marker::PhantomData;

use bevy::{prelude::*, ecs::component};

/// component for marking windows for visualizing unique components
#[derive(Component)]
pub struct Visualize<T: Component> {
    phantom: PhantomData<T>
}

impl<T: Component> Default for Visualize<T> {
    fn default() -> Self {
      Self { phantom: Default::default() }
    }
  }