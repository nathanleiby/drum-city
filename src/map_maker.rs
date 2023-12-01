use std::{fs::File, io::Write};

use crate::{
    consts::{AppState, MAP_MAKER_POSITION, START_TIME_OFFSET},
    time::ControlledTime,
    types::*,
};
use bevy::prelude::*;
use serde::Serialize;

#[derive(Component)]
struct MyMusic;

fn setup_audio(mut commands: Commands, audio: Res<MapMakerAudio>) {
    commands.spawn((
        AudioBundle {
            source: audio.0.clone(),
            settings: PlaybackSettings {
                paused: true,
                ..default()
            },
        },
        MyMusic,
    ));
}

fn start_song(time: Res<ControlledTime>, music_controller: Query<&AudioSink, With<MyMusic>>) {
    let secs = time.elapsed_seconds();
    let secs_last = secs - time.delta_seconds();

    // TOOD: without any delay this causes a panic b/c cannot find music_controller (`NoEntities`...)
    if secs_last <= START_TIME_OFFSET && START_TIME_OFFSET <= secs {
        let sink = music_controller
            .get_single()
            .expect("failed to get audio player");
        sink.play();
    }
}

#[derive(Resource)]
struct MapMakerAudio(Handle<AudioSource>);

const SONG_FILE: &str = "akisey-dance.ogg";

// The approach here it to create a handle to the material, so that arrows share a reference vs each having their own copy.
impl FromWorld for MapMakerAudio {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        let audio = asset_server.load(format!("songs/{}", SONG_FILE));
        Self(audio)
    }
}

#[derive(Resource, Serialize, Debug)]
struct Presses {
    arrows: Vec<ArrowTimeToml>,
    should_save: bool,
}

impl Drop for Presses {
    fn drop(&mut self) {
        if !self.should_save {
            return;
        }

        let out = SongConfigToml {
            name: "Map Maker output".to_string(),
            filename: SONG_FILE.to_string(),
            arrows: self.arrows.clone(),
        };
        let text = toml::to_string(&out).expect("Couldn't convert presses to toml text");
        let mut file = File::create("assets/songs/map.toml").expect("Couldn't open map.toml file");
        file.write_all(text.as_bytes())
            .expect("Couldn't write to map.toml file");
    }
}

fn setup_key_presses_storage(mut presses: ResMut<Presses>) {
    presses.should_save = true;
}

fn save_key_presses(
    time: Res<ControlledTime>,
    keyboard_input: Res<Input<KeyCode>>,
    mut presses: ResMut<Presses>,
) {
    let directions = [
        Directions::Up,
        Directions::Down,
        Directions::Left,
        Directions::Right,
    ];
    for direction in directions.iter() {
        if direction.key_just_pressed(&keyboard_input) {
            presses.arrows.push(ArrowTimeToml {
                click_time: time.elapsed_seconds_f64() - START_TIME_OFFSET as f64,
                speed: Speed::Slow,
                direction: *direction,
            });
        }
    }
}

// fn save_to_file_on_exit(mut event_reader: EventReader<AppExit>, presses: Res<Presses>) {
//     for event in event_reader.read() {
//         info!("event detected: {:?}", event);
//         let text = toml::to_string(&*presses).expect("Couldn't convert presses to toml text");
//         let mut file = File::create("assets/songs/map.toml").expect("Couldn't open map.toml file");
//         file.write_all(text.as_bytes())
//             .expect("Couldn't write to map.toml file");
//     }
// }

/// Keep textures for the arrow
#[derive(Resource)]
struct MapMakerArrowMaterialResource {
    border_image: Handle<Image>,
}

// The approach here it to create a handle to the material, so that arrows share a reference vs each having their own copy.
impl FromWorld for MapMakerArrowMaterialResource {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        let border_handle = asset_server.load("images/arrow_border.png");

        MapMakerArrowMaterialResource {
            border_image: border_handle,
        }
    }
}

#[derive(Component)]
struct MapMakerArrow(Directions);

fn setup_map_maker_arrows(mut commands: Commands, materials: Res<MapMakerArrowMaterialResource>) {
    for direction in &[
        Directions::Up,
        Directions::Down,
        Directions::Left,
        Directions::Right,
    ] {
        let mut transform =
            Transform::from_translation(Vec3::new(MAP_MAKER_POSITION, direction.y(), 1.));
        transform.rotate(Quat::from_rotation_z(direction.rotation()));

        commands
            .spawn(SpriteBundle {
                texture: materials.border_image.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(140., 140.)),
                    ..Default::default()
                },
                transform,
                ..Default::default()
            })
            .insert(MapMakerArrow(*direction));
    }
}

fn toggle_map_maker_arrows(
    mut query: Query<(&MapMakerArrow, &mut Visibility)>,
    input: Res<Input<KeyCode>>,
) {
    for (arrow, mut visible) in query.iter_mut() {
        if arrow.0.key_pressed(&input) {
            *visible = Visibility::Visible;
        } else {
            *visible = Visibility::Hidden;
        }
    }
}

pub struct MapMakerPlugin;
impl Plugin for MapMakerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Presses {
            arrows: Vec::new(),
            should_save: false,
        })
        .init_resource::<MapMakerArrowMaterialResource>()
        .init_resource::<MapMakerAudio>()
        .add_systems(OnEnter(AppState::MakeMap), setup_audio)
        .add_systems(OnEnter(AppState::MakeMap), setup_key_presses_storage)
        // TODO: ideally this would run once on startup, but I can't figure out how to do that safely
        // with initializing the AudioSink. I tried flushing commands via `apply_deferred`
        // Hitting:
        //  failed to get audio player: NoEntities("bevy_ecs::query::state::QueryState<&bevy_audio::sinks::AudioSink, bevy_ecs::query::filter::With<drum_city::map_maker::MyMusic>>")
        .add_systems(Update, start_song.run_if(in_state(AppState::MakeMap)))
        .add_systems(Update, save_key_presses.run_if(in_state(AppState::MakeMap)))
        // .add_systems(
        //     Update,
        //     save_to_file_on_exit.run_if(in_state(AppState::MakeMap)),
        // )
        .add_systems(OnEnter(AppState::MakeMap), setup_map_maker_arrows)
        .add_systems(
            Update,
            toggle_map_maker_arrows.run_if(in_state(AppState::MakeMap)),
        );
    }
}
