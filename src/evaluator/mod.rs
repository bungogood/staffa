use bkgm::{Dice, GameResult, State};
use std::fmt;
use std::fmt::Formatter;

mod hyper;
mod onnx;
mod ply;
mod pubeval;
pub use hyper::HyperEvaluator;
pub use onnx::OnnxEvaluator;
pub use ply::PlyEvaluator;
pub use pubeval::PubEval;

/// Sum of all six fields will always be 1.0
#[derive(PartialEq)]
pub struct Probabilities {
    pub win_normal: f32,
    pub win_gammon: f32,
    pub win_bg: f32,
    pub lose_normal: f32,
    pub lose_gammon: f32,
    pub lose_bg: f32,
}

impl fmt::Debug for Probabilities {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Probabilities: wn {:.2}%; wg {:.2}%; wb {:.2}%; ln {:.2}%; lg {:.2}%; lb {:.2}%",
            100.0 * self.win_normal,
            100.0 * self.win_gammon,
            100.0 * self.win_bg,
            100.0 * self.lose_normal,
            100.0 * self.lose_gammon,
            100.0 * self.lose_bg
        )
    }
}

/// Used when writing CSV data to a file
impl fmt::Display for Probabilities {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{};{};{};{};{};{}",
            self.win_normal,
            self.win_gammon,
            self.win_bg,
            self.lose_normal,
            self.lose_gammon,
            self.lose_bg
        )
    }
}

impl Probabilities {
    /// Typically used from rollouts.
    /// The index within the array has to correspond to the discriminant of the `Probabilities` enum.
    /// Input integer values will be normalized so that the sum in the return value is 1.0
    pub fn new(results: &[u32; 6]) -> Self {
        let sum = results.iter().sum::<u32>() as f32;
        Probabilities {
            win_normal: results[GameResult::WinNormal as usize] as f32 / sum,
            win_gammon: results[GameResult::WinGammon as usize] as f32 / sum,
            win_bg: results[GameResult::WinBackgammon as usize] as f32 / sum,
            lose_normal: results[GameResult::LoseNormal as usize] as f32 / sum,
            lose_gammon: results[GameResult::LoseGammon as usize] as f32 / sum,
            lose_bg: results[GameResult::LoseBackgammon as usize] as f32 / sum,
        }
    }

    pub fn from(results: &GameResult) -> Self {
        match results {
            GameResult::WinNormal => Self {
                win_normal: 1.0,
                win_gammon: 0.0,
                win_bg: 0.0,
                lose_normal: 0.0,
                lose_gammon: 0.0,
                lose_bg: 0.0,
            },
            GameResult::WinGammon => Self {
                win_normal: 0.0,
                win_gammon: 1.0,
                win_bg: 0.0,
                lose_normal: 0.0,
                lose_gammon: 0.0,
                lose_bg: 0.0,
            },
            GameResult::WinBackgammon => Self {
                win_normal: 0.0,
                win_gammon: 0.0,
                win_bg: 1.0,
                lose_normal: 0.0,
                lose_gammon: 0.0,
                lose_bg: 0.0,
            },
            GameResult::LoseNormal => Self {
                win_normal: 0.0,
                win_gammon: 0.0,
                win_bg: 1.0,
                lose_normal: 1.0,
                lose_gammon: 0.0,
                lose_bg: 0.0,
            },
            GameResult::LoseGammon => Self {
                win_normal: 0.0,
                win_gammon: 0.0,
                win_bg: 0.0,
                lose_normal: 0.0,
                lose_gammon: 1.0,
                lose_bg: 0.0,
            },
            GameResult::LoseBackgammon => Self {
                win_normal: 0.0,
                win_gammon: 0.0,
                win_bg: 0.0,
                lose_normal: 0.0,
                lose_gammon: 0.0,
                lose_bg: 1.0,
            },
        }
    }

    pub fn normalized(&self) -> Self {
        let sum = self.to_vec().iter().sum::<f32>();
        Probabilities {
            win_normal: self.win_normal / sum,
            win_gammon: self.win_gammon / sum,
            win_bg: self.win_bg / sum,
            lose_normal: self.lose_normal / sum,
            lose_gammon: self.lose_gammon / sum,
            lose_bg: self.lose_bg / sum,
        }
    }

    fn switch_sides(&self) -> Self {
        Self {
            win_normal: self.lose_normal,
            win_gammon: self.lose_gammon,
            win_bg: self.lose_bg,
            lose_normal: self.win_normal,
            lose_gammon: self.win_gammon,
            lose_bg: self.win_bg,
        }
    }

    /// Cubeless equity
    pub fn equity(&self) -> f32 {
        self.win_normal - self.lose_normal
            + 2.0 * (self.win_gammon - self.lose_gammon)
            + 3.0 * (self.win_bg - self.lose_bg)
    }

    pub fn to_vec(&self) -> Vec<f32> {
        vec![
            self.win_normal,
            self.win_gammon,
            self.win_bg,
            self.lose_normal,
            self.lose_gammon,
            self.lose_bg,
        ]
    }
}

pub trait Evaluator<T: State>: Sized {
    /// Returns a cubeless evaluation of a position.
    /// Implementing types will calculate the probabilities with different strategies.
    /// Examples of such strategies are a rollout or 1-ply inference of a neural net.
    fn eval(&self, pos: &T) -> f32;

    /// Returns the position after applying the *best* move to `pos`.
    /// The returned `Position` has already switches sides.
    /// This means the returned position will have the *lowest* equity of possible positions.
    fn best_position(&self, pos: &T, dice: &Dice) -> T {
        self.worst_position(&pos.possible_positions(dice)).clone()
    }

    /// Worst position might be interesting, because when you switch sides, it's suddenly the best.
    fn worst_position<'a>(&'a self, positions: &'a [T]) -> &T {
        positions
            .iter()
            .map(|pos| (pos, self.eval(pos)))
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap()
            .0
    }
}

pub struct RandomEvaluator {}

impl<G: State> Evaluator<G> for RandomEvaluator {
    #[allow(dead_code)]
    /// Returns random probabilities. Each call will return different values.
    fn eval(&self, pos: &G) -> f32 {
        fastrand::f32()
    }

    fn best_position(&self, pos: &G, dice: &Dice) -> G {
        self.worst_position(&pos.possible_positions(dice)).clone()
    }

    fn worst_position<'a>(&'a self, positions: &'a [G]) -> &G {
        positions
            .iter()
            .map(|pos| (pos, self.eval(pos)))
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap()
            .0
    }
}

impl RandomEvaluator {
    pub fn new() -> RandomEvaluator {
        #[allow(dead_code)]
        RandomEvaluator {}
    }
}

#[cfg(test)]
mod probabilities_tests {
    use crate::evaluator::Probabilities;

    #[test]
    fn new() {
        // sum of `results is 32, a power of 2. Makes fractions easier to handle.
        let results = [0_u32, 1, 3, 4, 8, 16];
        let probabilities = Probabilities::new(&results);
        assert_eq!(probabilities.win_normal, 0.0);
        assert_eq!(probabilities.win_gammon, 0.03125);
        assert_eq!(probabilities.win_bg, 0.09375);
        assert_eq!(probabilities.lose_normal, 0.125);
        assert_eq!(probabilities.lose_gammon, 0.25);
        assert_eq!(probabilities.lose_bg, 0.5);
    }

    #[test]
    fn to_string() {
        let probabilities = Probabilities {
            win_normal: 1.0 / 21.0,
            win_gammon: 2.0 / 21.0,
            win_bg: 3.0 / 21.0,
            lose_normal: 4.0 / 21.0,
            lose_gammon: 5.0 / 21.0,
            lose_bg: 6.0 / 21.0,
        };
        assert_eq!(
            probabilities.to_string(),
            "0.04761905;0.0952381;0.14285715;0.1904762;0.23809524;0.2857143"
        );
    }

    #[test]
    fn equity_win_normal() {
        let probabilities = Probabilities {
            win_normal: 1.0,
            win_gammon: 0.0,
            win_bg: 0.0,
            lose_normal: 0.0,
            lose_gammon: 0.0,
            lose_bg: 0.0,
        };
        assert_eq!(probabilities.equity(), 1.0);
    }

    #[test]
    fn equity_win_gammon() {
        let probabilities = Probabilities {
            win_normal: 0.0,
            win_gammon: 1.0,
            win_bg: 0.0,
            lose_normal: 0.0,
            lose_gammon: 0.0,
            lose_bg: 0.0,
        };
        assert_eq!(probabilities.equity(), 2.0);
    }

    #[test]
    fn equity_win_bg() {
        let probabilities = Probabilities {
            win_normal: 0.0,
            win_gammon: 0.0,
            win_bg: 1.0,
            lose_normal: 0.0,
            lose_gammon: 0.0,
            lose_bg: 0.0,
        };
        assert_eq!(probabilities.equity(), 3.0);
    }

    #[test]
    fn equity_lose_normal() {
        let probabilities = Probabilities {
            win_normal: 0.0,
            win_gammon: 0.0,
            win_bg: 0.0,
            lose_normal: 1.0,
            lose_gammon: 0.0,
            lose_bg: 0.0,
        };
        assert_eq!(probabilities.equity(), -1.0);
    }

    #[test]
    fn equity_lose_gammon() {
        let probabilities = Probabilities {
            win_normal: 0.0,
            win_gammon: 0.0,
            win_bg: 0.0,
            lose_normal: 0.0,
            lose_gammon: 1.0,
            lose_bg: 0.0,
        };
        assert_eq!(probabilities.equity(), -2.0);
    }

    #[test]
    fn equity_lose_bg() {
        let probabilities = Probabilities {
            win_normal: 0.0,
            win_gammon: 0.0,
            win_bg: 0.0,
            lose_normal: 0.0,
            lose_gammon: 0.0,
            lose_bg: 1.0,
        };
        assert_eq!(probabilities.equity(), -3.0);
    }

    #[test]
    fn equity_balanced() {
        let probabilities = Probabilities {
            win_normal: 0.3,
            win_gammon: 0.1,
            win_bg: 0.1,
            lose_normal: 0.3,
            lose_gammon: 0.1,
            lose_bg: 0.1,
        };
        assert_eq!(probabilities.equity(), 0.0);
    }
}
#[cfg(test)]
mod evaluator_trait_tests {
    use crate::evaluator::Evaluator;
    use bkgm::{bpos, Backgammon, Dice, State};

    fn expected_pos() -> Backgammon {
        bpos!(x 5:1, 3:1; o 20:2).flip()
    }

    /// Test double. Returns not so good probabilities for `expected_pos`, better for everything else.
    struct EvaluatorFake {}
    impl Evaluator<Backgammon> for EvaluatorFake {
        fn eval(&self, pos: &Backgammon) -> f32 {
            if pos == &expected_pos() {
                1.0
            } else {
                -1.0
            }
        }
    }

    #[test]
    fn best_position() {
        // Given
        let given_pos = bpos!(x 7:2; o 20:2);
        let evaluator = EvaluatorFake {};
        // When
        let best_pos = evaluator.best_position(&given_pos, &Dice::new(4, 2));
        // Then
        assert_eq!(best_pos, expected_pos());
    }
}
