use crate::consts::*;
use bevy::{
    prelude::*,
    utils::{Duration, Instant},
};

#[derive(Resource)]
pub struct ControlledTime {
    delta: Duration,
    last_update: Option<Instant>,
    delta_seconds_f64: f64,
    delta_seconds: f32,
    seconds_since_startup: f64,
    startup: Instant,
}
impl Default for ControlledTime {
    fn default() -> Self {
        Self {
            delta: Duration::from_secs(0),
            last_update: None,
            startup: Instant::now(),
            delta_seconds_f64: 0.0,
            seconds_since_startup: 0.0,
            delta_seconds: 0.0,
        }
    }
}

// TODO: port to how 0.12 Bevy thinks of Time (Time::new_with might also help)
impl ControlledTime {
    pub fn reset_time(&mut self) {
        self.startup = Instant::now();
        self.seconds_since_startup = 0.0;
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        self.update_with_instant(now);
    }

    pub fn update_with_instant(&mut self, instant: Instant) {
        if let Some(last_update) = self.last_update {
            self.delta = instant - last_update;
            self.delta_seconds_f64 = self.delta.as_secs_f64();
            self.delta_seconds = self.delta.as_secs_f32();
        }

        let duration_since_startup = instant - self.startup;
        self.seconds_since_startup = duration_since_startup.as_secs_f64();
        self.last_update = Some(instant);
    }

    /// The delta between the current and last tick as [`f32`] seconds
    #[inline]
    pub fn delta_seconds(&self) -> f32 {
        self.delta_seconds
    }

    /// The delta between the current and last tick as [`f64`] seconds
    #[inline]
    pub fn delta_seconds_f64(&self) -> f64 {
        self.delta_seconds_f64
    }

    /// The time since startup in seconds
    #[inline]
    pub fn elapsed_seconds(&self) -> f32 {
        self.seconds_since_startup as f32
    }

    /// The time since startup in seconds
    #[inline]
    pub fn elapsed_seconds_f64(&self) -> f64 {
        self.seconds_since_startup
    }
}

// ----- //

pub fn update_time(mut time: ResMut<ControlledTime>) {
    time.update();
}

pub fn reset_time_when_entering_game(mut time: ResMut<ControlledTime>) {
    time.reset_time();
}

pub struct TimePlugin;
impl Plugin for TimePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ControlledTime>()
            // Game
            .add_systems(OnEnter(AppState::Game), reset_time_when_entering_game)
            .add_systems(Update, update_time.run_if(in_state(AppState::Game)))
            // MakeMap
            .add_systems(OnEnter(AppState::MakeMap), reset_time_when_entering_game)
            .add_systems(Update, update_time.run_if(in_state(AppState::MakeMap)));
    }
}
