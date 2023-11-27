use bevy::{
    prelude::*,
    window::{PresentMode, WindowResolution},
};

mod arrows;
use arrows::ArrowsPlugin;

mod score;
use score::Score;

mod ui;
use ui::UIPlugin;

mod consts;
mod types;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Resource)]
struct GreetTimer(Timer);

fn main() {
    let config = types::load_config();
    App::new()
        .add_systems(Update, bevy::window::close_on_esc)
        // antialiasing
        .insert_resource(Msaa::Sample4)
        // song config
        .insert_resource(config)
        // score tracking
        .insert_resource(Score::new())
        // window configuration
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Drum City!".to_string(),
                resolution: WindowResolution::new(800., 600.),
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(CameraPlugin)
        .add_plugins(ArrowsPlugin)
        .add_plugins(UIPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}
