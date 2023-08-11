use bevy::prelude::{Component};

/// marks component as a valid candidate for serialization. 
#[derive(Component)]
pub struct Serializable;