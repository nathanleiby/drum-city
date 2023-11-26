use bevy::{
    prelude::*,
    window::{PresentMode, WindowResolution},
};

mod arrows;
use arrows::ArrowsPlugin;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Resource)]
struct GreetTimer(Timer);

fn main() {
    App::new()
        .insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        // antialiasing
        .insert_resource(Msaa::Sample4)
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
        .add_systems(Update, bevy::window::close_on_esc)
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
