use bevy::prelude::*;

use crate::consts::AppState;

pub struct DebugPlugin;

fn print_app_state_changes(app_state: Res<State<AppState>>) {
    if app_state.is_changed() {
        info!("Changed app state: {:?}", app_state);
    }
}

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, print_app_state_changes);
    }
}
