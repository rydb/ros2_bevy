use bevy::prelude::*;

/// states that thing is already visualized, so no more tugs are needed.
#[derive(Component)]
pub struct Visualized;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct MeshPull {
    /// the position indexes that bind together to make a vertex. [`Mesh::ATTRIBUTE_POSITION`], that this pull affects . !!!If this index doesn't exist, this may cause a panic!!!
    //pub position_indexs: [usize; 3],
    pub position_index: usize,
    /// handle of mesh to pull.
    pub mesh_handle: Handle<Mesh>
}