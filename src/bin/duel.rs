use bkgm::Backgammon;
use clap::Parser;
use rayon::prelude::*;
use staffa::dice::FastrandDice;
use staffa::evaluator::{self, HyperEvaluator, OnnxEvaluator, RandomEvaluator};
use staffa::probabilities::{Probabilities, ResultCounter};
use staffa::{duel::Duel, evaluator::PubEval};
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
    #[arg(short = 'm', long = "matches", default_value = "10000")]
    matches: usize,
}

fn run(args: &Args) {
    // let evaluator1 = HyperEvaluator::new().unwrap();
    let evaluator1 =
        OnnxEvaluator::<Backgammon>::from_file_path(&args.model1).expect("Model not found");
    let evaluator2 = OnnxEvaluator::from_file_path(&args.model2).expect("Model not found");
    // let evaluator2 = PubEval::new();
    // let evaluator2 = RandomEvaluator::new();
    let duel = Duel::new(evaluator1, evaluator2);

    // TODO: play args.matches games and print the results
    // TODO: add a progress bar & eta while keeping results on the same line
    // println!("Let two Evaluators duel each other:");
    // for _ in 0..args.matches {
    //     duel.duel_once();
    //     let probabilities = duel.probabilities();
    //     print!(
    //         "\rAfter {} games is the equity {:.3}. {:?}",
    //         duel.number_of_games(),
    //         probabilities.equity(),
    //         probabilities,
    //     );
    //     stdout().flush().unwrap()
    // }
    // println!("\nDone");

    let mut global_counter = ResultCounter::default();

    loop {
        // If we create n seeds, than n duels are played in parallel which gives us 2*n GameResults.
        // When the duels have finished, the 2*n results are reduced to a single one and then
        // added to the `global_counter`.
        // Those global results are printed out and the endless loop starts again.
        let seeds: Vec<u64> = (0..50).map(|x| x).collect();
        let counter = seeds
            .par_iter()
            .map(|seed| duel.duel(&mut FastrandDice::with_seed(*seed)))
            .reduce(ResultCounter::default, |a, b| a.combine(&b));

        global_counter = global_counter.combine(&counter);
        let probabilities = Probabilities::from(&global_counter);
        let better_evaluator = if probabilities.equity() > 0.0 { 1 } else { 2 };
        print!(
            "\rEvaluator {} is leading. After {:.1} thousand games the equity is {:.3}. {:?}",
            better_evaluator,
            global_counter.sum() as f32 / 1000.0,
            probabilities.equity(),
            probabilities,
        );
        stdout().flush().unwrap();
    }
}

fn main() {
    let args = Args::parse();
    run(&args);
}
