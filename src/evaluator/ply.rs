use crate::evaluator::Probabilities;

use super::Evaluator;
use bkgm::{dice::ALL_21, Position};

pub struct PlyEvaluator<T: Evaluator> {
    evaluator: T,
    depth: usize,
}

impl<T: Evaluator + Sync> Evaluator for PlyEvaluator<T> {
    fn eval(&self, pos: &Position) -> Probabilities {
        match pos.game_state() {
            bkgm::GameState::Ongoing => self.ply(pos, self.depth),
            bkgm::GameState::GameOver(result) => Probabilities::from(&result),
        }
    }
}

impl<T: Evaluator> PlyEvaluator<T> {
    pub fn new(evaluator: T, depth: usize) -> Self {
        Self { evaluator, depth }
    }

    // TODO: Implement N ply
    pub fn ply(&self, pos: &Position, depth: usize) -> Probabilities {
        let mut probs = Probabilities::blank();
        for (dice, n) in ALL_21 {
            let best = self.evaluator.best_position(pos, &dice);
            let nprob = self.evaluator.eval(&best);
            probs = Probabilities {
                win_normal: probs.win_normal + nprob.win_normal * n,
                win_gammon: probs.win_gammon + nprob.win_gammon * n,
                win_bg: probs.win_bg + nprob.win_bg * n,
                lose_normal: probs.lose_normal + nprob.lose_normal * n,
                lose_gammon: probs.lose_gammon + nprob.lose_gammon * n,
                lose_bg: probs.lose_bg + nprob.lose_bg * n,
            }
        }
        probs.normalized().switch_sides()
    }
}
