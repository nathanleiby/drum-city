use crate::consts::*;
use bevy::{
    ecs::system::Resource,
    input::{keyboard::KeyCode, Input},
};
use core::f32::consts::PI;

#[derive(Copy, Clone, Debug, PartialEq)]
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

#[derive(Copy, Clone, Debug)]
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

#[derive(Debug, Resource)]
pub struct SongConfig {
    pub arrows: Vec<ArrowTime>,
}
impl ArrowTime {
    fn new(click_time: f64, speed: Speed, direction: Directions) -> Self {
        Self {
            spawn_time: click_time - (DISTANCE / speed.value()) as f64,
            speed,
            direction,
        }
    }
}

pub fn load_config() -> SongConfig {
    SongConfig {
        arrows: vec![
            ArrowTime::new(1.0, Speed::Slow, Directions::Up),
            ArrowTime::new(2.0, Speed::Slow, Directions::Down),
            ArrowTime::new(3.0, Speed::Slow, Directions::Left),
            ArrowTime::new(4.0, Speed::Medium, Directions::Up),
            ArrowTime::new(5.0, Speed::Fast, Directions::Right),
        ],
    }
}
