use crate::evaluator::Evaluator;
use bkgm::{utils::mcomb, Hypergammon, State};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const POSSIBLE: usize = mcomb(26, Hypergammon::NUM_CHECKERS as usize).pow(2);

#[derive(Clone)]
pub struct HyperEvaluator {
    equities: Vec<f32>,
}

impl Evaluator<Hypergammon> for HyperEvaluator {
    fn eval(&self, position: &Hypergammon) -> f32 {
        self.equities[position.dbhash()]
    }
}

impl HyperEvaluator {
    pub fn new() -> Option<Self> {
        Self::from_file("data/hyper.db")
    }

    pub fn from_file(file_path: impl AsRef<Path>) -> Option<Self> {
        let file = File::open(file_path).expect("File not found");
        let equities: Vec<f32> = F32Reader::new(BufReader::new(file)).collect();
        if equities.len() == POSSIBLE {
            Some(Self { equities })
        } else {
            None
        }
    }
}

struct F32Reader<R: BufRead> {
    inner: R,
}

impl<R: BufRead> F32Reader<R> {
    pub fn new(inner: R) -> Self {
        Self { inner }
    }
}

impl<R: BufRead> Iterator for F32Reader<R> {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buff: [u8; 4] = [0; 4];
        self.inner.read_exact(&mut buff).ok()?;
        Some(f32::from_le_bytes(buff))
    }
}
