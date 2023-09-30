use bkgm::Position;
use staffa::evaluator::{Evaluator, Probabilities, RandomEvaluator};
use staffa::inputs::Inputs;
use staffa::onnx::OnnxEvaluator;
use staffa::position_finder::PositionFinder;
use staffa::rollout::RolloutEvaluator;
use std::fs::File;
use std::io::{stdout, Write};
use std::time::Instant;

const AMOUNT: usize = 1_000;

fn main() -> std::io::Result<()> {
    let path = "data/rollouts.csv";
    println!("Roll out and write CSV data to {}", path);
    _ = std::fs::create_dir("data");
    _ = std::fs::remove_file(path);
    let mut file = File::create(path)?;
    file.write_all(csv_header().as_bytes())?;

    let evaluator =
        OnnxEvaluator::from_file_path("model/staffa.onnx").map(RolloutEvaluator::with_evaluator);
    let finder = OnnxEvaluator::from_file_path("model/staffa.onnx").map(PositionFinder::new);
    let start = Instant::now();

    match (evaluator, finder) {
        (Some(evaluator), Some(mut finder)) => {
            println!("Use onnx evaluator");
            let positions = finder.find_positions(AMOUNT);
            for (i, position) in positions.iter().enumerate() {
                let probabilities = evaluator.eval(position);
                write_csv_line(&mut file, position, &probabilities, i, start)?;
            }
        }
        (_, _) => {
            println!("Couldn't find onnx file, use random evaluator");
            let evaluator = RolloutEvaluator::with_evaluator(RandomEvaluator {});
            let mut finder = PositionFinder::new(RandomEvaluator {});
            let positions = finder.find_positions(AMOUNT);
            for (i, position) in positions.iter().enumerate() {
                let probabilities = evaluator.eval(position);
                write_csv_line(&mut file, position, &probabilities, i, start)?;
            }
        }
    }
    println!("\nDone!");
    Ok(())
}

fn write_csv_line(
    file: &mut File,
    position: &Position,
    probabilities: &Probabilities,
    i: usize,
    start: Instant,
) -> std::io::Result<()> {
    file.write_all(csv_line(position, probabilities).as_bytes())?;
    let done = (i + 1) as f32 / AMOUNT as f32;
    let todo = 1.0 - done;
    let seconds_done = start.elapsed().as_secs();
    let seconds_todo = (seconds_done as f32 * (todo / done)) as u64;
    print!(
        "\rProgress: {:2.2} %. Time elapsed: {}. Time left: {}.",
        done * 100.0,
        duration(seconds_done),
        duration(seconds_todo),
    );
    stdout().flush().unwrap();
    Ok(())
}

fn duration(seconds: u64) -> String {
    let minutes = seconds / 60;
    let hours = minutes / 60;
    let minutes = minutes % 60;
    let seconds = seconds % 60;
    format!("{:02}:{:02}:{:02} h", hours, minutes, seconds)
}

fn csv_header() -> String {
    Probabilities::csv_header() + ";" + Inputs::csv_header().as_str() + "\n"
}

fn csv_line(position: &Position, probabilities: &Probabilities) -> String {
    probabilities.to_string() + ";" + Inputs::from_position(position).to_string().as_str() + "\n"
}
