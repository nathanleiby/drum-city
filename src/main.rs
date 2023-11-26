use bevy::{
    prelude::*,
    window::{CursorGrabMode, PresentMode, WindowResolution},
};

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
        .add_plugins(HelloPlugin)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

fn greet_people(
    time: Res<Time>,
    mut greet_timer: ResMut<GreetTimer>,
    query: Query<&Name, With<Person>>,
) {
    // add elapsed time to timer.
    // if it just finished an interval, then print
    if greet_timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}

pub struct HelloPlugin;
impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_people)
            .add_systems(Update, greet_people);
    }
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
