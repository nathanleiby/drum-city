use crate::consts::*;
use bevy::{
    asset::{AssetServer, Handle},
    audio::AudioSource,
    ecs::{
        system::Resource,
        world::{FromWorld, World},
    },
    input::{keyboard::KeyCode, Input},
};
use core::f32::consts::PI;
use serde_derive::{Deserialize, Serialize};
use std::{fs::File, io::Read};

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Directions {
    Up,
    Down,
    Left,
    Right,
}

impl Directions {
    /// Checks if the key corresponding to this direction was just pressed
    pub fn key_just_pressed(&self, input: &Input<KeyCode>) -> bool {
        match self {
            Directions::Up => input.just_pressed(KeyCode::Up),
            Directions::Down => input.just_pressed(KeyCode::Down),
            Directions::Left => input.just_pressed(KeyCode::Left),
            Directions::Right => input.just_pressed(KeyCode::Right),
        }
    }

    /// Returns the correct rotation for an arrow with this direction
    pub fn rotation(&self) -> f32 {
        match self {
            Directions::Up => PI * 0.5,
            Directions::Down => -PI * 0.5,
            Directions::Left => PI,
            Directions::Right => 0.,
        }
    }

    /// Returns the correct y-coordinate for an arrow with this direction
    pub fn y(&self) -> f32 {
        match self {
            Directions::Up => 150.,
            Directions::Down => 50.,
            Directions::Left => -50.,
            Directions::Right => -150.,
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Speed {
    Slow,
    Medium,
    Fast,
}

impl Speed {
    /// Returns the correct speed for an arrow with this speed
    pub fn value(&self) -> f32 {
        BASE_SPEED * self.multiplier()
    }

    /// Speed multiplier for an arrow with this speed
    pub fn multiplier(&self) -> f32 {
        match self {
            Speed::Slow => 1.,
            Speed::Medium => 1.2,
            Speed::Fast => 1.5,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ArrowTime {
    pub spawn_time: f64,
    pub speed: Speed,
    pub direction: Directions,
}

impl ArrowTime {
    fn new_from_toml(a: &ArrowTimeToml) -> Self {
        Self {
            spawn_time: a.click_time - (DISTANCE / a.speed.value()) as f64,
            speed: a.speed,
            direction: a.direction,
        }
    }
}

#[derive(Debug, Resource)]
pub struct SongConfig {
    pub name: String,
    pub song_audio: Handle<AudioSource>,
    pub arrows: Vec<ArrowTime>,
}

pub fn load_config(path: &str, asset_server: &AssetServer) -> SongConfig {
    // For WASM, fetch a remote file
    // https://rustwasm.github.io/wasm-bindgen/examples/fetch.html
    let mut file = File::open(format!("assets/songs/{}", path)).expect("Could not open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Could not read file into string");

    let parsed: SongConfigToml =
        toml::from_str(&contents).expect("Could not parse into SongConfigToml");

    let mut arrows = parsed
        .arrows
        .iter()
        .map(|a| ArrowTime::new_from_toml(a))
        .collect::<Vec<ArrowTime>>();

    // Sort by spawn_time
    arrows.sort_by(|a, b| a.spawn_time.partial_cmp(&b.spawn_time).unwrap());

    // TODO: what is &* about
    let song_audio = asset_server.load(format!("songs/{}", parsed.filename));

    SongConfig {
        name: parsed.name,
        arrows,
        song_audio,
    }
}

// The approach here it to create a handle to the material, so that arrows share a reference vs each having their own copy.
impl FromWorld for SongConfig {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();

        load_config("akisey-dance.toml", asset_server)
    }
}

#[derive(Deserialize, Serialize)]
pub struct SongConfigToml {
    pub name: String,
    pub filename: String,
    pub arrows: Vec<ArrowTimeToml>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ArrowTimeToml {
    pub click_time: f64,
    pub speed: Speed,
    pub direction: Directions,
}
