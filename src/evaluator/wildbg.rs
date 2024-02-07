use crate::evaluator::{Evaluator, NNEvaluator, PartialEvaluator};
use crate::probabilities::Probabilities;
use bkgm::State;
use std::marker::PhantomData;
use std::path::Path;
use tract_onnx::prelude::*;

#[derive(Clone)]
pub struct WildbgEvaluator<G: State> {
    #[allow(clippy::type_complexity)]
    model: RunnableModel<TypedFact, Box<dyn TypedOp>, Graph<TypedFact, Box<dyn TypedOp>>>,
    phantom: PhantomData<G>,
}

impl<G: State> PartialEvaluator<G> for WildbgEvaluator<G> {
    fn try_eval(&self, pos: &G) -> f32 {
        let probs = self.eval(pos);
        probs.equity()
    }
}

impl<G: State> Evaluator<G> for WildbgEvaluator<G> {
    fn eval(&self, position: &G) -> Probabilities {
        let output = self.output_vec(position);
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

impl<G: State> NNEvaluator<G> for WildbgEvaluator<G> {
    const MODEL_PATH: &'static str = "neural-nets/wildbg.onnx";
    const NUM_INTPUTS: usize = 202;
    const NUM_OUTPUTS: usize = 6;

    fn from_file_path(file_path: impl AsRef<Path>) -> Option<Self> {
        let model = onnx()
            .model_for_path(file_path)
            .unwrap()
            .into_optimized()
            .unwrap()
            .into_runnable()
            .unwrap();
        Some(Self {
            model,
            phantom: PhantomData,
        })
    }

    fn input_labels(&self) -> Vec<String> {
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

    fn output_labels(&self) -> Vec<String> {
        vec![
            "win_normal".to_string(),
            "win_gammon".to_string(),
            "win_bg".to_string(),
            "lose_normal".to_string(),
            "lose_gammon".to_string(),
            "lose_bg".to_string(),
        ]
    }

    fn input_vec(&self, position: &G) -> Vec<f32> {
        let mut x_inputs = [NO_CHECKERS; 25];
        let mut o_inputs = [NO_CHECKERS; 25];
        // on the bar:
        x_inputs[0] = PipInput::from_pip(position.x_bar());
        o_inputs[0] = PipInput::from_pip(position.o_bar());
        // on the board:
        for i in 1..=24 {
            let pip = position.pip(i);
            #[allow(clippy::comparison_chain)]
            if pip > 0 {
                x_inputs[i] = PipInput::from_pip(pip as u8);
            } else if pip < 0 {
                o_inputs[i] = PipInput::from_pip(-pip as u8);
            }
        }

        let mut vec: Vec<f32> = Vec::with_capacity(Self::NUM_INTPUTS);
        vec.push(position.x_off() as f32);
        vec.push(position.o_off() as f32);
        for input in x_inputs {
            vec.push(input.p1 as f32);
            vec.push(input.p2 as f32);
            vec.push(input.p3 as f32);
            vec.push(input.p4 as f32);
        }
        for input in o_inputs {
            vec.push(input.p1 as f32);
            vec.push(input.p2 as f32);
            vec.push(input.p3 as f32);
            vec.push(input.p4 as f32);
        }
        vec
    }

    fn output_vec(&self, position: &G) -> Vec<f32> {
        let inputs = self.input_vec(position);
        let tract_inputs = tract_ndarray::Array1::from_vec(inputs)
            .into_shape([1, Self::NUM_INTPUTS])
            .unwrap();
        let tensor = tract_inputs.into_tensor();

        // run the model on the input
        let result = self.model.run(tvec!(tensor.into())).unwrap();
        let array_view = result[0].to_array_view::<f32>().unwrap();
        array_view.iter().map(|&x| x).collect()
    }
}

struct PipInput {
    p1: u8,
    p2: u8,
    p3: u8,
    p4: u8,
}

const NO_CHECKERS: PipInput = PipInput {
    p1: 0,
    p2: 0,
    p3: 0,
    p4: 0,
};

impl PipInput {
    fn from_pip(pip: u8) -> Self {
        match pip {
            0 => NO_CHECKERS,
            1 => Self {
                p1: 1,
                p2: 0,
                p3: 0,
                p4: 0,
            },
            2 => Self {
                p1: 0,
                p2: 1,
                p3: 0,
                p4: 0,
            },
            p => Self {
                p1: 0,
                p2: 0,
                p3: 1,
                p4: p - 3,
            },
        }
    }
}
