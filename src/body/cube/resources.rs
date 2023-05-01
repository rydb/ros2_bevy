
const CUBE_SPAWN_TIMER: f32 = 0.5;

use bevy::prelude::*;
///Sample timer for setting spawn
#[derive(Resource)]
pub struct CubeTimer {
    /// track when the bomb should explode (non-repeating timer)
     pub timer: Timer,
}

impl Default for CubeTimer {
    fn default() -> CubeTimer {
        CubeTimer { timer: Timer::from_seconds(CUBE_SPAWN_TIMER, TimerMode::Once) ,}
    }
}