use std::marker::PhantomData;

use crate::dice::{DiceGen, FastrandDice};
use crate::evaluator::{Evaluator, Probabilities};
use bkgm::GameState::{GameOver, Ongoing};
use bkgm::State;

pub struct Duel<T: Evaluator<G>, U: Evaluator<G>, G: State> {
    evaluator1: T,
    evaluator2: U,
    dice_gen: FastrandDice,
    results: [u32; 6],
    phantom: PhantomData<G>,
}

/// Let two `Evaluator`s duel each other. A bit quick and dirty.
impl<T: Evaluator<G>, U: Evaluator<G>, G: State> Duel<T, U, G> {
    #[allow(clippy::new_without_default)]
    pub fn new(evaluator1: T, evaluator2: U) -> Self {
        Self::with_dice_gen(evaluator1, evaluator2, FastrandDice::new())
    }

    fn with_dice_gen(evaluator1: T, evaluator2: U, dice_gen: FastrandDice) -> Self {
        Duel {
            evaluator1,
            evaluator2,
            dice_gen,
            results: [0; 6],
            phantom: PhantomData,
        }
    }

    pub fn number_of_games(&self) -> u32 {
        self.results.iter().sum()
    }

    pub fn probabilities(&self) -> Probabilities {
        Probabilities::new(&self.results)
    }

    /// The two `Evaluator`s will play twice each against each other.
    /// Either `Evaluator` will start once and play with the same dice as vice versa.
    pub fn duel_once(&mut self) {
        let mut pos1 = G::new();
        let mut pos2 = G::new();
        let mut iteration = 0;
        let mut pos1_finished = false;
        let mut pos2_finished = false;
        while !(pos1_finished && pos2_finished) {
            let dice = self.dice_gen.roll();

            match pos1.game_state() {
                Ongoing => {
                    pos1 = if iteration % 2 == 0 {
                        self.evaluator1.best_position(&pos1, &dice)
                    } else {
                        self.evaluator2.best_position(&pos1, &dice)
                    };
                }
                GameOver(result) => {
                    if !pos1_finished {
                        pos1_finished = true;
                        let result = if iteration % 2 == 0 {
                            result
                        } else {
                            result.reverse()
                        };
                        self.results[result as usize] += 1;
                    }
                }
            }
            match pos2.game_state() {
                Ongoing => {
                    pos2 = if iteration % 2 == 0 {
                        self.evaluator2.best_position(&pos2, &dice)
                    } else {
                        self.evaluator1.best_position(&pos2, &dice)
                    };
                }
                GameOver(result) => {
                    if !pos2_finished {
                        pos2_finished = true;
                        let result = if iteration % 2 == 0 {
                            result.reverse()
                        } else {
                            result
                        };
                        self.results[result as usize] += 1;
                    }
                }
            }
            iteration += 1;
        }
    }
}
