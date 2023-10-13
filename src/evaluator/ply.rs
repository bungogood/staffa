use crate::evaluator::Probabilities;

use super::Evaluator;
use bkgm::{dice::ALL_21, Position};

pub struct PlyEvaluator<T: Evaluator> {
    evaluator: T,
    depth: usize,
}

impl<T: Evaluator + Sync> Evaluator for PlyEvaluator<T> {
    fn eval(&self, pos: &Position) -> f32 {
        match pos.game_state() {
            bkgm::GameState::Ongoing => self.ply(pos, self.depth),
            bkgm::GameState::GameOver(result) => result.value(),
        }
    }
}

impl<T: Evaluator> PlyEvaluator<T> {
    pub fn new(evaluator: T, depth: usize) -> Self {
        Self { evaluator, depth }
    }

    // TODO: Implement N ply
    pub fn ply(&self, pos: &Position, depth: usize) -> f32 {
        let mut score = 0.0;
        for (dice, n) in ALL_21 {
            let best = self.evaluator.best_position(pos, &dice);
            score += n * self.evaluator.eval(&best);
        }
        score / 36.0
    }
}
