use bevy::prelude::*;
use std::time::Duration;

/// general countdown timer
#[derive(Resource)]
pub struct CountDownTimer{
    pub timer: Timer,
}

impl CountDownTimer {
    pub fn new(countdown: u64) -> Self {
        Self { timer: Timer::new(Duration::from_secs(countdown), TimerMode::Once)}
    }
}