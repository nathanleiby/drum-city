use bevy::ecs::system::Resource;

use crate::consts::THRESHOLD;

#[derive(Resource)]
pub struct Score {
    score: usize,

    corrects: usize,
    fails: usize,
}

impl Score {
    pub fn new() -> Self {
        Score {
            score: 0,
            corrects: 0,
            fails: 0,
        }
    }

    // increments the number of corrects, updates the players total score, and returns the number of points earned
    pub fn incr_correct(&mut self, distance: f32) -> usize {
        self.corrects += 1;

        let score_multiplier = (THRESHOLD - distance.abs()) / THRESHOLD;
        let points = (100.0 * score_multiplier).min(100.).max(10.) as usize;
        self.score += points;

        points
    }

    pub fn incr_failed(&mut self) {
        self.fails += 1;
    }

    // Getters
    pub fn get_score(&self) -> usize {
        self.score
    }

    pub fn get_corrects(&self) -> usize {
        self.corrects
    }

    pub fn get_fails(&self) -> usize {
        self.fails
    }
}
