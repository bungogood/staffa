use bkgm::{pos, Dice, Position};
use staffa::evaluator::Evaluator;
use staffa::onnx::OnnxEvaluator;
use std::collections::HashMap;

fn main() {
    let onnx = OnnxEvaluator::with_default_model().unwrap();

    let position = Position::new();
    let best = onnx.best_position(&position, &Dice::new(3, 1));
    println!("best after rolling 31: {:?}", best.flip());

    let best = onnx.best_position(&position, &Dice::new(6, 1));
    println!("best after rolling 61: {:?}", best.flip());

    let position = pos!(x 5:1, 3:4; o 24:3);
    let best = onnx.best_position(&position, &Dice::new(4, 3));
    println!("best in bearoff after 43 {:?}", best.flip());
}
