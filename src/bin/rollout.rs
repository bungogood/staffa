use bkgm::Position;
use staffa::evaluator::{Evaluator, Probabilities};
use staffa::onnx::OnnxEvaluator;
use staffa::position_finder::PositionFinder;
use staffa::rollout::RolloutEvaluator;
use std::fs::File;
use std::io::{stdout, Write};
use std::time::Instant;

const DATA_DIR: &str = "data";

const AMOUNT: usize = 10;
const SEP: &str = ",";

fn main() -> std::io::Result<()> {
    let path = format!("{}/rollouts.csv", DATA_DIR);
    println!("Roll out and write CSV data to {}", path);
    _ = std::fs::create_dir(DATA_DIR);
    _ = std::fs::remove_file(&path);
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
        (_, _) => panic!("Could not load onnx evaluator"),
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
    let headers = vec![
        "positionid",
        "win_normal",
        "win_gammon",
        "win_bg",
        "lose_normal",
        "lose_gammon",
        "lose_bg",
    ];
    headers.join(SEP) + "\n"
}

fn csv_line(position: &Position, prob: &Probabilities) -> String {
    let probs = vec![
        prob.win_normal,
        prob.win_gammon,
        prob.win_bg,
        prob.lose_normal,
        prob.lose_gammon,
        prob.lose_bg,
    ];
    let probstr = probs
        .iter()
        .map(|f| format!("{:.5}", f))
        .collect::<Vec<String>>()
        .join(SEP);
    position.position_id() + SEP + &probstr + "\n"
}
