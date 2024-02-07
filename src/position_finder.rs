use crate::dice::{DiceGen, FastrandDice};
use crate::evaluator::Evaluator;
use bkgm::GameState::Ongoing;
use bkgm::State;
use std::collections::HashSet;
use std::hash::Hash;
use std::marker::PhantomData;

/// Finds random positions for later rollout.
pub struct PositionFinder<E: Evaluator<G>, G: State> {
    evaluator: E,
    dice_gen: FastrandDice,
    phantom: PhantomData<G>,
}

impl<E: Evaluator<G>, G: State + Eq + Hash> PositionFinder<E, G> {
    /// Contains different random number generators every time it's called.
    #[allow(clippy::new_without_default)]
    pub fn new(evaluator: E) -> Self {
        PositionFinder {
            evaluator,
            dice_gen: FastrandDice::new(),
            phantom: PhantomData,
        }
    }

    pub fn find_positions(&mut self, amount: usize) -> HashSet<G> {
        let mut found: HashSet<G> = HashSet::new();
        while found.len() < amount {
            let mut more = self.positions_in_one_random_game();
            while found.len() < amount {
                match more.pop() {
                    Some(pos) => found.insert(pos),
                    None => break,
                };
            }
        }
        found
    }

    fn positions_in_one_random_game(&mut self) -> Vec<G> {
        let mut positions: Vec<G> = Vec::new();
        let mut pos = G::new();
        while pos.game_state() == Ongoing {
            // Todo: Don't allow doubles in first move
            let dice = self.dice_gen.roll();
            let pos = self.evaluator.best_position(&pos, &dice);
            let new_positions = pos.possible_positions(&dice);
            // Todo: remove cloning by implementing the Copy trait -> maybe better performance
            // pos = self.evaluator.worst_position(&new_positions).clone();
            let mut ongoing_games: Vec<G> = new_positions
                .into_iter()
                .filter(|p| p.game_state() == Ongoing)
                .collect();
            positions.append(&mut ongoing_games);
        }
        positions
    }
}
