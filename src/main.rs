use std::io::{stdout, Write};
use std::ops::Add;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use bkgm::Position;

trait TrainedEvaluator {
    fn inputs(position: &Position) -> Vec<f32>;
    fn inputs_headers() -> Vec<&'static str>;
    fn output_headers() -> Vec<&'static str>;
}

const NO_CHECKERS: PipInput = PipInput {
    p1: 0,
    p2: 0,
    p3: 0,
    p4: 0,
};

struct PipInput {
    p1: u8,
    p2: u8,
    p3: u8,
    p4: u8,
}

impl PipInput {
    pub fn new(pip: u8) -> Self {
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

struct WildBG {}

impl TrainedEvaluator for WildBG {
    fn inputs(position: &Position) -> Vec<f32> {
        let mut out = vec![];
        out.push(position.x_off() as f32);
        out.push(position.o_off() as f32);
        for point in 1..=24 {
            let pip = position.pip(point);
            let pip = if pip > 0 {
                PipInput::new(pip as u8)
            } else {
                NO_CHECKERS
            };
            out.push(pip.p1 as f32);
            out.push(pip.p2 as f32);
            out.push(pip.p3 as f32);
            out.push(pip.p4 as f32);
        }
        let pip = position.x_bar();
        let pip = if pip > 0 {
            PipInput::new(pip as u8)
        } else {
            NO_CHECKERS
        };
        out.push(pip.p1 as f32);
        out.push(pip.p2 as f32);
        out.push(pip.p3 as f32);
        out.push(pip.p4 as f32);
        for point in 1..=24 {
            let pip = position.pip(point);
            let pip = if pip < 0 {
                PipInput::new(-pip as u8)
            } else {
                NO_CHECKERS
            };
            out.push(pip.p1 as f32);
            out.push(pip.p2 as f32);
            out.push(pip.p3 as f32);
            out.push(pip.p4 as f32);
        }
        let pip = position.o_bar();
        let pip = if pip > 0 {
            PipInput::new(pip as u8)
        } else {
            NO_CHECKERS
        };
        out.push(pip.p1 as f32);
        out.push(pip.p2 as f32);
        out.push(pip.p3 as f32);
        out.push(pip.p4 as f32);
        out
    }

    fn inputs_headers() -> Vec<&'static str> {
        vec![
            "x_off", "o_off", "x_bar-1", "x_bar-2", "x_bar-3", "x_bar-4", "x1-1", "x1-2", "x1-3",
            "x1-4", "x2-1", "x2-2", "x2-3", "x2-4", "x3-1", "x3-2", "x3-3", "x3-4", "x4-1", "x4-2",
            "x4-3", "x4-4", "x5-1", "x5-2", "x5-3", "x5-4", "x6-1", "x6-2", "x6-3", "x6-4", "x7-1",
            "x7-2", "x7-3", "x7-4", "x8-1", "x8-2", "x8-3", "x8-4", "x9-1", "x9-2", "x9-3", "x9-4",
            "x10-1", "x10-2", "x10-3", "x10-4", "x11-1", "x11-2", "x11-3", "x11-4", "x12-1",
            "x12-2", "x12-3", "x12-4", "x13-1", "x13-2", "x13-3", "x13-4", "x14-1", "x14-2",
            "x14-3", "x14-4", "x15-1", "x15-2", "x15-3", "x15-4", "x16-1", "x16-2", "x16-3",
            "x16-4", "x17-1", "x17-2", "x17-3", "x17-4", "x18-1", "x18-2", "x18-3", "x18-4",
            "x19-1", "x19-2", "x19-3", "x19-4", "x20-1", "x20-2", "x20-3", "x20-4", "x21-1",
            "x21-2", "x21-3", "x21-4", "x22-1", "x22-2", "x22-3", "x22-4", "x23-1", "x23-2",
            "x23-3", "x23-4", "x24-1", "x24-2", "x24-3", "x24-4", "o_bar-1", "o_bar-2", "o_bar-3",
            "o_bar-4", "o1-1", "o1-2", "o1-3", "o1-4", "o2-1", "o2-2", "o2-3", "o2-4", "o3-1",
            "o3-2", "o3-3", "o3-4", "o4-1", "o4-2", "o4-3", "o4-4", "o5-1", "o5-2", "o5-3", "o5-4",
            "o6-1", "o6-2", "o6-3", "o6-4", "o7-1", "o7-2", "o7-3", "o7-4", "o8-1", "o8-2", "o8-3",
            "o8-4", "o9-1", "o9-2", "o9-3", "o9-4", "o10-1", "o10-2", "o10-3", "o10-4", "o11-1",
            "o11-2", "o11-3", "o11-4", "o12-1", "o12-2", "o12-3", "o12-4", "o13-1", "o13-2",
            "o13-3", "o13-4", "o14-1", "o14-2", "o14-3", "o14-4", "o15-1", "o15-2", "o15-3",
            "o15-4", "o16-1", "o16-2", "o16-3", "o16-4", "o17-1", "o17-2", "o17-3", "o17-4",
            "o18-1", "o18-2", "o18-3", "o18-4", "o19-1", "o19-2", "o19-3", "o19-4", "o20-1",
            "o20-2", "o20-3", "o20-4", "o21-1", "o21-2", "o21-3", "o21-4", "o22-1", "o22-2",
            "o22-3", "o22-4", "o23-1", "o23-2", "o23-3", "o23-4", "o24-1", "o24-2", "o24-3",
            "o24-4H",
        ]
        // for i in 1..=24 {
        //     "x{i}-1"
        //     "x{i}-2"
        //     "x{i}-3"
        //     "x{i}-4"
        // }
    }

    fn output_headers() -> Vec<&'static str> {
        vec!["win", "wing", "winbg", "loseg", "losebg"]
    }
}

fn write_headers(file: &mut File) {
    let mut headers = WildBG::inputs_headers();
    headers.append(&mut WildBG::output_headers());
    let outstr = headers.join(",").add("\n");
    file.write(outstr.as_bytes()).unwrap();
}

fn write_line(file: &mut File, position: &Position, outputs: Vec<f32>) {
    let mut out: Vec<f32> = outputs.clone();
    out.append(&mut WildBG::inputs(position));
    let outstr = out
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",")
        .add("\n");
    file.write(outstr.as_bytes()).unwrap();
}

fn run(inpath: &str, outpath: &str) {
    let infile = File::open(inpath).unwrap();
    let reader = BufReader::new(infile);

    let mut lines_iter = reader.lines().peekable();
    lines_iter.next();

    let mut outfile = File::create(outpath).unwrap();

    write_headers(&mut outfile);

    for line in lines_iter {
        match line {
            Ok(line) => {
                let mut fields = line.split(',');
                let position = fields.next().unwrap();
                let outputs = fields.map(|x| x.parse::<f32>().unwrap()).collect();
                let position = Position::from_id(position.to_string());
                write_line(&mut outfile, &position, outputs)
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn main() {
    run("data/contact-train-data.csv", "data/data-out.csv");
    run("data/small.csv", "data/small-out.csv");
}
