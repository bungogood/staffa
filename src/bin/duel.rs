use clap::Parser;
use staffa::duel::Duel;
use staffa::onnx::OnnxEvaluator;
use std::{
    io::{stdout, Write},
    path::PathBuf,
};

// TODO: improve argument names & allow for rollouts, random, etc.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Model 1
    model1: PathBuf,

    /// Model 2
    model2: PathBuf,

    /// Matches
    #[arg(short = 'm', long = "matches", default_value = "100")]
    matches: usize,
}

fn run(args: &Args) {
    let evaluator1 = OnnxEvaluator::from_file_path(&args.model1).unwrap();
    let evaluator2 = OnnxEvaluator::from_file_path(&args.model2).unwrap();
    let mut duel = Duel::new(evaluator1, evaluator2);

    // TODO: play args.matches games and print the results
    // TODO: add a progress bar & eta while keeping results on the same line
    println!("Let two Evaluators duel each other:");
    for _ in 0..args.matches {
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

fn main() {
    let args = Args::parse();
    run(&args);
}
