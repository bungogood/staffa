use bkgm::Position;
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use staffa::evaluator::{Evaluator, OnnxEvaluator};
use std::io;
use std::path::PathBuf;

/// Benchmark the model evaluation speed

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Model file
    #[arg(short = 'm', long = "model", default_value = "model/staffa.onnx")]
    model: PathBuf,

    /// Number of games to generate
    #[arg(short = 'n', long = "num-positions", default_value = "1000000")]
    num_positions: usize,
}

fn run(args: &Args) -> io::Result<()> {
    let evaluator = OnnxEvaluator::from_file_path(&args.model).expect("Model not found");

    let pb = ProgressBar::new(args.num_positions as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{wide_bar} {pos}/{len} ({percent}%) Elapsed: {elapsed_precise} ETA: {eta_precise}",
            )
            .unwrap(),
    );

    let position = Position::new();
    for _ in 0..args.num_positions {
        evaluator.eval(&position);
        pb.inc(1);
    }

    pb.finish_and_clear();
    let dur = pb.elapsed();
    let speed = args.num_positions as f64 / dur.as_secs_f64();
    let avg_time = dur / args.num_positions as u32;
    println!(
        "Elapsed: {:.2?}, Positions: {} Speed: {:.2}/s, Avg: {:.2?}",
        dur, args.num_positions, speed, avg_time
    );
    Ok(())
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    run(&args)
}
