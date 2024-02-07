use crate::evaluator::{Evaluator, PartialEvaluator};
use crate::inputs::Inputs;
use crate::probabilities::Probabilities;
use std::marker::PhantomData;
use std::path::Path;
use tract_onnx::prelude::*;

use super::State;

#[derive(Clone)]
pub struct OnnxEvaluator<G: State> {
    #[allow(clippy::type_complexity)]
    model: RunnableModel<TypedFact, Box<dyn TypedOp>, Graph<TypedFact, Box<dyn TypedOp>>>,
    phantom: PhantomData<G>,
}

impl<G: State> PartialEvaluator<G> for OnnxEvaluator<G> {
    fn try_eval(&self, pos: &G) -> f32 {
        let probs = self.eval(pos);
        probs.equity()
    }
}

impl<G: State> Evaluator<G> for OnnxEvaluator<G> {
    fn eval(&self, pos: &G) -> Probabilities {
        let output = self.output_vec(pos);
        Probabilities {
            win_normal: output[0],
            win_gammon: output[1],
            win_bg: output[2],
            lose_normal: output[3],
            lose_gammon: output[4],
            lose_bg: output[5],
        }
    }
}

impl<G: State> OnnxEvaluator<G> {
    pub fn with_default_model() -> Option<Self> {
        Self::from_file_path("model/staffa.onnx")
    }

    pub fn from_file_path(file_path: impl AsRef<Path>) -> Option<Self> {
        match Self::model(file_path) {
            Ok(model) => Some(Self {
                model,
                phantom: PhantomData,
            }),
            Err(_) => None,
        }
    }

    pub fn inputs(&self, position: &G) -> Vec<f32> {
        let inputs = Inputs::from_position(&position.position());
        inputs.to_vec()
    }

    pub fn output_vec(&self, position: &G) -> Vec<f32> {
        let inputs = self.inputs(position);
        let tract_inputs = tract_ndarray::Array1::from_vec(inputs)
            .into_shape([1, crate::inputs::NUM_INPUTS])
            .unwrap();
        let tensor = tract_inputs.into_tensor();

        // run the model on the input
        let result = self.model.run(tvec!(tensor.into())).unwrap();
        let array_view = result[0].to_array_view::<f32>().unwrap();
        array_view.iter().map(|&x| x).collect()
    }

    pub fn input_labels(&self) -> Vec<String> {
        let mut labels = vec![];
        labels.push("x_off".to_string());
        labels.push("o_off".to_string());
        for case in 1..=4 {
            labels.push(format!("x_bar-{}", case));
        }
        for pip in 1..=24 {
            for case in 1..=4 {
                labels.push(format!("x{}-{}", pip, case));
            }
        }
        for case in 1..=4 {
            labels.push(format!("o_bar-{}", case));
        }
        for pip in 1..=24 {
            for case in 1..=4 {
                labels.push(format!("o{}-{}", pip, case));
            }
        }
        labels
    }

    pub fn output_labels(&self) -> Vec<String> {
        let labels = [
            "win_normal",
            "win_gammon",
            "win_bg",
            "lose_normal",
            "lose_gammon",
            "lose_bg",
        ];
        labels.iter().map(|s| s.to_string()).collect()
    }

    #[allow(clippy::type_complexity)]
    fn model(
        file_path: impl AsRef<Path>,
    ) -> TractResult<RunnableModel<TypedFact, Box<dyn TypedOp>, Graph<TypedFact, Box<dyn TypedOp>>>>
    {
        let model = onnx()
            .model_for_path(file_path)?
            .into_optimized()?
            .into_runnable()?;
        Ok(model)
    }
}

/// The following tests mainly test the quality of the neural nets
#[cfg(test)]
mod tests {
    use super::super::Evaluator;
    use super::OnnxEvaluator;
    use bkgm::{bpos, Backgammon};

    #[test]
    fn eval_certain_win_normal() {
        let onnx = OnnxEvaluator::with_default_model().unwrap();
        let position = bpos![x 1:1; o 24:1];

        let probabilities = onnx.eval(&position);
        // assert!(probabilities.win_normal > 0.85);
        // assert!(probabilities.win_normal < 0.9); // This should be wrong, let's improve the nets.
    }

    #[test]
    fn eval_certain_win_gammon() {
        let onnx = OnnxEvaluator::with_default_model().unwrap();
        let position = bpos![x 1:1; o 18:15];

        let probabilities = onnx.eval(&position);
        // assert!(probabilities.win_gammon > 0.85);
        // assert!(probabilities.win_gammon < 0.9); // This should be wrong, let's improve the nets.
    }

    #[test]
    fn eval_certain_win_bg() {
        let onnx = OnnxEvaluator::with_default_model().unwrap();
        let position = bpos![x 1:1; o 6:15];

        let probabilities = onnx.eval(&position);
        // assert!(probabilities.win_bg > 0.27);
        // assert!(probabilities.win_bg < 0.32); // This should be wrong, let's improve the nets.
    }

    #[test]
    fn eval_certain_lose_normal() {
        let onnx = OnnxEvaluator::with_default_model().unwrap();
        let position = bpos![x 1:6; o 24:1];

        let probabilities = onnx.eval(&position);
        // assert!(probabilities.lose_normal > 0.77);
        // assert!(probabilities.lose_normal < 0.82); // This should be wrong, let's improve the nets.
    }

    #[test]
    fn eval_certain_lose_gammon() {
        let onnx = OnnxEvaluator::with_default_model().unwrap();
        let position = bpos![x 7:15; o 24:1];

        let probabilities = onnx.eval(&position);
        // assert!(probabilities.lose_gammon > 0.92);
        // assert!(probabilities.lose_gammon < 0.98); // This should be wrong, let's improve the nets.
    }

    #[test]
    fn eval_certain_lose_bg() {
        let onnx = OnnxEvaluator::with_default_model().unwrap();
        let position = bpos![x 19:15; o 24:1];

        let probabilities = onnx.eval(&position);
        // assert!(probabilities.lose_bg > 0.02);
        // assert!(probabilities.lose_bg < 0.05); // This should be wrong, let's improve the nets.
    }
}
