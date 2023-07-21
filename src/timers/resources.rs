
use bevy::prelude::*;


#[derive(Component)]
pub struct DespawnTimer {
    pub timer: Timer,
}

impl DespawnTimer {
    pub fn new(seconds: f32) -> Self {
        Self {
            timer: Timer::from_seconds(seconds, TimerMode::Once)
        }
    }
}

/// tick tiemr to despawn enemies, and despawn them if timer has reached zero.
pub fn tick_despawn_timer(
    time: Res<Time>,
    mut commands: Commands,
    mut timer_query: Query<(Entity, &mut DespawnTimer)>,
) {
    for (e, mut timer) in timer_query.iter_mut() {
        //println!("ticking timer");
        timer.timer.tick(time.delta());

        if timer.timer.finished() {
            //println!("despawning particle");
            commands.entity(e).despawn();
        }
    }
}