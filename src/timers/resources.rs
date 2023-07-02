
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

        if(timer.timer.finished()) {
            //println!("despawning particle");
            commands.entity(e).despawn();
        }
    }
}


// timer that counts down
// #[derive(Component)]
// pub struct CountDownTimer {
//     pub timer: Timer,
// }

// impl CountDownTimer {
//     pub fn new(seconds: f32) -> Self {
//         Self {
//             timer: Timer::from_seconds(seconds, TimerMode::Once)
//         }
//     }
// }

// pub fn tick_countdown_timer(
//     time: Res<Time>,
//     mut countdown_timer_query: Query<&CountDownTimer>,
// ) {
//     for countdown_timer in countdown_timer_query.iter_mut() {
//         countdown_timer.timer.tick(time.delta());
//     }
// }

// impl Default for CountDownTimer {
//     // fn default() -> CountDownTimer {
//     //     CountDownTimer { timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating) ,}
//     // }
// }