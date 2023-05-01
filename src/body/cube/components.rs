use bevy::prelude::*;
use crate::body::cube::resources::*;
use crate::body::cube::systems::*;


//use self::systems::{spawn_cubes_over_time, tick_cube_spawn_timer};
///plugin for spawning in cubes over time. Used for testing physics.
pub struct CubePlugin;


impl Plugin for CubePlugin {
    fn build(&self, app: &mut App){
        app
        .init_resource::<CubeTimer>()
        .add_startup_system(spawn_cube)
        //.add_system(tick_cube_spawn_timer)

        
        ;
    }
}