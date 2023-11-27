use bevy::prelude::*;

use crate::{consts::START_TIME_OFFSET, types::SongConfig};

#[derive(Component)]
struct MyMusic;

fn setup(mut commands: Commands, song_config: Res<SongConfig>) {
    commands.spawn((
        AudioBundle {
            source: song_config.song_audio.clone(),
            settings: PlaybackSettings {
                paused: true,
                ..default()
            },
        },
        MyMusic,
    ));
}

fn start_song(time: Res<Time>, music_controller: Query<&AudioSink, With<MyMusic>>) {
    let secs = time.elapsed_seconds();
    let secs_last = secs - time.delta_seconds();

    if secs_last <= START_TIME_OFFSET && START_TIME_OFFSET <= secs {
        let sink = music_controller
            .get_single()
            .expect("failed to get audio player");
        sink.play();
    }
}

pub struct AudioPlugin;
impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, start_song);
    }
}
