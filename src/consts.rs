use bevy::prelude::*;

// Speed at which a Slow arrow moves
pub const BASE_SPEED: f32 = 200.;

/// X coordinate value where arrows spaw. should be off the screen.
pub const SPAWN_POSITION: f32 = -400.;

/// X coordinate value where arrows should be clicked
pub const TARGET_POSITION: f32 = 200.;

/// Margin of error (in x coordinate val) for clicking on an arrow
pub const THRESHOLD: f32 = 20.;

/// Total distance between the spawn and target positions
pub const DISTANCE: f32 = TARGET_POSITION - SPAWN_POSITION;

/// Number of seconds to wait before audio plays
pub const START_TIME_OFFSET: f32 = 3.;

/// Width of the window
pub const WINDOW_WIDTH: f32 = 800.;

/// Height of the window
pub const WINDOW_HEIGHT: f32 = 600.;

/// App state -- manages where we are in the game
pub const APP_STATE_STAGE: &str = "app_state_stage";

/// States
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Menu,
    Game,
    MakeMap,
}
