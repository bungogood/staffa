use staffa::duel::Duel;
use staffa::onnx::OnnxEvaluator;
use std::io::{stdout, Write};

fn main() {
    // let evaluator1 = OnnxEvaluator::from_file_path("model/wildbg.onnx").unwrap();
    let evaluator1 = OnnxEvaluator::from_file_path("model/staffa.onnx").unwrap();
    // let evaluator2 = RandomEvaluator {};
    let evaluator2 = OnnxEvaluator::from_file_path("model/wildbg.onnx").unwrap();
    // let evaluator2 = OnnxEvaluator::from_file_path("model/wildbg.onnx").unwrap();
    // let evaluator2 = OnnxEvaluator::from_file_path("model/wildbg01.onnx").unwrap();
    let mut duel = Duel::new(evaluator1, evaluator2);

    println!("Let two Evaluators duel each other:");
    for _ in 0..100_000 {
        duel.duel_once();
        let probabilities = duel.probabilities();
        print!(
            "\rAfter {} games is the equity {:.3}. {:?}",
            duel.number_of_games(),
            probabilities.equity(),
            probabilities,
        );
        stdout().flush().unwrap()
    }
    println!("\nDone");
}
