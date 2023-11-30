use bevy::{
    prelude::*,
    window::{PresentMode, WindowResolution},
};

mod arrows;
mod menu;
use arrows::ArrowsPlugin;

mod score;
use consts::*;
use menu::MenuPlugin;
use score::Score;

mod ui;
use ui::UIPlugin;

mod audio;
use audio::AudioPlugin;

mod consts;
mod types;

mod shaders;
use shaders::ShadersPlugin;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Resource)]
struct GreetTimer(Timer);

fn main() {
    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
        .add_systems(Update, bevy::window::close_on_esc)
        // antialiasing
        .insert_resource(Msaa::Sample4)
        // score tracking
        .insert_resource(Score::new())
        // window configuration
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Drum City!".to_string(),
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        // .insert_resource(State::new(AppState::Menu))
        .add_state::<AppState>()
        .add_plugins(CameraPlugin)
        .add_plugins(ArrowsPlugin)
        .add_plugins(UIPlugin)
        .add_plugins(AudioPlugin)
        .add_plugins(ShadersPlugin)
        .add_plugins(MenuPlugin)
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
