use bkgm::{Dice, State};

mod hyper;
mod onnx;
mod ply;
mod pubeval;
pub use hyper::HyperEvaluator;
pub use onnx::OnnxEvaluator;
pub use ply::PlyEvaluator;
pub use pubeval::PubEval;

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
