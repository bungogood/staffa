use std::marker::PhantomData;

use crate::probabilities::Probabilities;

use super::Evaluator;
use bkgm::{dice::ALL_21, State};

pub struct PlyEvaluator<E: Evaluator<G>, G: State> {
    evaluator: E,
    #[allow(dead_code)]
    depth: usize,
    phantom: PhantomData<G>,
}

impl<E: Evaluator<G>, G: State> Evaluator<G> for PlyEvaluator<E, G> {
    fn eval(&self, pos: &G) -> Probabilities {
        match pos.game_state() {
            bkgm::GameState::Ongoing => self.ply(pos),
            bkgm::GameState::GameOver(result) => result.value(),
        };
        todo!()
    }
}

impl<E: Evaluator<G>, G: State> PlyEvaluator<E, G> {
    pub fn new(evaluator: E, depth: usize) -> Self {
        Self {
            evaluator,
            depth,
            phantom: PhantomData,
        }
    }

    // TODO: Implement N ply
    pub fn ply(&self, pos: &G) -> f32 {
        let mut score = 0.0;
        for (dice, n) in ALL_21 {
            let best = self.evaluator.best_position(pos, &dice);
            // score += n * self.evaluator.eval(&best);
        }
        score / 36.0
    }
}
