use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use staffa::evaluator::Evaluator;
use staffa::onnx::OnnxEvaluator;
use staffa::position_finder::PositionFinder;
use staffa::rollout::RolloutEvaluator;
use std::fs::File;
use std::io;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Output file
    outfile: PathBuf,

    /// Model file
    #[arg(short = 'm', long = "model", default_value = "model/staffa.onnx")]
    model: PathBuf,

    /// Number of games to generate
    #[arg(short = 'n', long = "num-positions", default_value = "1000")]
    num_positions: usize,

    /// Separator
    #[arg(short = 's', long = "sep", default_value = ",")]
    sep: char, // TODO: Fix this to be a single byte and accept ;
}

fn run(args: &Args) -> io::Result<()> {
    // add error handling
    let evaluator = OnnxEvaluator::from_file_path(&args.model).unwrap();

    let mut headers = vec!["positionid".to_string()];
    headers.extend(evaluator.output_labels());

    let rollout = RolloutEvaluator::with_evaluator(evaluator.clone());
    let mut finder = PositionFinder::new(evaluator);

    let outfile = File::create(&args.outfile)?;

    let mut wtr = csv::WriterBuilder::new()
        .delimiter(args.sep as u8)
        .from_writer(outfile);

    wtr.write_record(headers)?;

    let pb = ProgressBar::new(args.num_positions as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{wide_bar} {pos}/{len} ({percent}%) Elapsed: {elapsed_precise} ETA: {eta_precise}",
            )
            .unwrap(),
    );

    let positions = finder.find_positions(args.num_positions);
    for position in positions.iter() {
        let probabilities = rollout.eval(position);
        let mut data = vec![position.position_id().to_string()];
        data.extend(probabilities.to_vec().iter().map(|f| format!("{:.5}", f)));
        wtr.write_record(data).unwrap();
        pb.inc(1);
    }

    pb.finish_and_clear();
    let dur = pb.elapsed();
    println!("Positions: {}", args.num_positions);
    println!("Elapsed: {:.2?}", dur);
    Ok(())
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    run(&args)
}
