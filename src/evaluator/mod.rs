use bkgm::{Dice, State};
use std::path::Path;

mod hyper;
mod onnx;
mod ply;
// mod pubeval;
mod wildbg;
use crate::probabilities::Probabilities;
pub use hyper::HyperEvaluator;
pub use onnx::OnnxEvaluator;
pub use ply::PlyEvaluator;
// pub use pubeval::PubEval;
pub use wildbg::WildbgEvaluator;

pub trait PartialEvaluator<G: State>: Sized {
    /// Returns a cubeless evaluation of a position.
    /// Implementing types will calculate the probabilities with different strategies.
    /// Examples of such strategies are a rollout or 1-ply inference of a neural net.
    fn try_eval(&self, pos: &G) -> f32;
}

pub trait Evaluator<G: State>: Sized {
    /// Returns a cubeless evaluation of a position.
    /// Implementing types will calculate the probabilities with different strategies.
    /// Examples of such strategies are a rollout or 1-ply inference of a neural net.
    fn eval(&self, pos: &G) -> Probabilities;

    /// Returns the position after applying the *best* move to `pos`.
    /// The returned `Position` has already switches sides.
    /// This means the returned position will have the *lowest* equity of possible positions.
    fn best_position(&self, pos: &G, dice: &Dice) -> G {
        self.worst_position(&pos.possible_positions(dice)).clone()
    }

    /// Worst position might be interesting, because when you switch sides, it's suddenly the best.
    fn worst_position<'a>(&'a self, positions: &'a [G]) -> &G {
        positions
            .iter()
            .map(|pos| (pos, self.eval(pos).equity()))
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap()
            .0
    }
}

pub trait NNEvaluator<G: State>: Evaluator<G> + Sized {
    const MODEL_PATH: &'static str;
    const NUM_INTPUTS: usize;
    const NUM_OUTPUTS: usize;

    fn with_default_model() -> Option<Self> {
        Self::from_file_path(Self::MODEL_PATH)
    }

    fn from_file_path(file_path: impl AsRef<Path>) -> Option<Self>;

    fn input_labels(&self) -> Vec<String>;
    fn output_labels(&self) -> Vec<String>;

    fn input_vec(&self, position: &G) -> Vec<f32>;
    fn output_vec(&self, position: &G) -> Vec<f32>;
}

pub struct RandomEvaluator;

impl<G: State> Evaluator<G> for RandomEvaluator {
    #[allow(dead_code)]
    /// Returns random probabilities. Each call will return different values.
    fn eval(&self, pos: &G) -> Probabilities {
        let win_normal = fastrand::f32();
        let win_gammon = fastrand::f32();
        let win_bg = fastrand::f32();
        let lose_normal = fastrand::f32();
        let lose_gammon = fastrand::f32();
        let lose_bg = fastrand::f32();

        // Now we like to make sure that the different probabilities add up to 1
        let sum = win_normal + win_gammon + win_bg + lose_normal + lose_gammon + lose_bg;
        Probabilities {
            win_normal: win_normal / sum,
            win_gammon: win_gammon / sum,
            win_bg: win_bg / sum,
            lose_normal: lose_normal / sum,
            lose_gammon: lose_gammon / sum,
            lose_bg: lose_bg / sum,
        }
    }
}

impl RandomEvaluator {
    pub fn new() -> RandomEvaluator {
        #[allow(dead_code)]
        RandomEvaluator {}
    }
}

#[cfg(test)]
mod evaluator_trait_tests {
    use crate::evaluator::PartialEvaluator;
    use bkgm::{bpos, Backgammon, Dice, State};

    fn expected_pos() -> Backgammon {
        bpos!(x 5:1, 3:1; o 20:2).flip()
    }

    // Test double. Returns not so good probabilities for `expected_pos`, better for everything else.
    // struct EvaluatorFake {}
    // impl PartialEvaluator<Backgammon> for EvaluatorFake {
    //     fn eval(&self, pos: &Backgammon) -> f32 {
    //         if pos == &expected_pos() {

    //         } else {
    //             -1.0
    //         }
    //     }
    // }

    // #[test]
    // fn best_position() {
    //     // Given
    //     let given_pos = bpos!(x 7:2; o 20:2);
    //     let evaluator = EvaluatorFake {};
    //     // When
    //     let best_pos = evaluator.best_position(&given_pos, &Dice::new(4, 2));
    //     // Then
    //     assert_eq!(best_pos, expected_pos());
    // }
}
