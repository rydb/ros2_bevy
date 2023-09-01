use bevy::prelude::*;
use super::resources::*;
pub struct TimerManagerPlugin;

impl Plugin for TimerManagerPlugin {
    fn build(&self, app: &mut App) {
        app
        
        .add_systems(Update, tick_despawn_timer)
        ;
    }
}