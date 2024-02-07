use bkgm::{Dice, State};
use std::path::Path;

mod hyper;
// mod mcts;
mod onnx;
mod ply;
mod pubeval;
mod rollout;
mod wildbg;
use crate::probabilities::Probabilities;
pub use hyper::HyperEvaluator;
pub use onnx::OnnxEvaluator;
pub use ply::PlyEvaluator;
pub use pubeval::PubEval;
pub use rollout::RolloutEvaluator;
pub use wildbg::WildbgEvaluator;

pub trait PartialEvaluator<G: State>: Sized {
    /// Returns a cubeless evaluation of a position.
    /// Implementing types will calculate the probabilities with different strategies.
    /// Examples of such strategies are a rollout or 1-ply inference of a neural net.
    fn try_eval(&self, pos: &G) -> f32;

    fn best_position(&self, pos: &G, dice: &Dice) -> G {
        *pos.possible_positions(dice)
            .iter()
            .map(|pos| (pos, self.try_eval(&pos)))
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap()
            .0
    }
}

pub trait Evaluator<G: State>: PartialEvaluator<G> + Sized {
    /// Returns a cubeless evaluation of a position.
    /// Implementing types will calculate the probabilities with different strategies.
    /// Examples of such strategies are a rollout or 1-ply inference of a neural net.
    fn eval(&self, pos: &G) -> Probabilities;
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

impl<G: State> PartialEvaluator<G> for RandomEvaluator {
    fn try_eval(&self, pos: &G) -> f32 {
        let probs = self.eval(pos);
        probs.equity()
    }
}

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
