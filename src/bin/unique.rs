use std::collections::HashSet;

use bkgm::{
    dice::{ALL_21, ALL_SINGLES},
    Position,
};

/// Benchmark and test position / move generation

fn unqiue() -> HashSet<Position> {
    let position = Position::hypergammon();
    let mut found = HashSet::new();
    let mut new_positons = vec![];
    let before = found.len();
    for die in ALL_SINGLES {
        let children = position.all_positions_after_moving(&die);
        for child in children {
            if !found.contains(&child) {
                found.insert(child.clone());
                new_positons.push(child);
            }
        }
    }

    let mut depth = 1;
    let discovered = found.len() - before;
    println!(
        "{}\t{}\tpositions reached after {} roll",
        discovered,
        found.len(),
        depth
    );

    while !new_positons.is_empty() {
        let mut queue = new_positons;
        new_positons = vec![];
        let before = found.len();
        while let Some(position) = queue.pop() {
            for (die, _) in ALL_21 {
                let children = position.all_positions_after_moving(&die);
                for child in children {
                    if !found.contains(&child) {
                        found.insert(child.clone());
                        new_positons.push(child);
                    }
                }
            }
        }
        let discovered = found.len() - before;
        depth += 1;
        println!(
            "{}\t{}\tpositions reached after {} rolls",
            discovered,
            found.len(),
            depth
        );
    }

    found
}

fn main() {
    let start = std::time::Instant::now();
    let unique = unqiue();
    let total = unique.len();
    let dur = start.elapsed();
    let speed = total as f64 / dur.as_secs_f64();
    let avg_time = dur / total as u32;
    println!(
        "Elapsed: {:.2?}, Positions: {} Speed: {:.2}/s, Avg: {:.2?}",
        dur, total, speed, avg_time
    );
}
