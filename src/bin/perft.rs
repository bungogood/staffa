use bkgm::{
    Dice,
    GameState::{GameOver, Ongoing},
    Position,
};

const ALL_21: [Dice; 21] = all_21();

const fn all_21() -> [Dice; 21] {
    let mut dice = [Dice::Double(1); 21]; // Dummy values, will be replaced

    // for loops don't work with `const fn`
    let mut count = 0_usize;
    let mut i = 0_usize;
    while i < 6 {
        let mut j = i;
        while j < 6 {
            dice[count] = Dice::new(i + 1, j + 1);
            j += 1;
            count += 1;
        }
        i += 1;
    }
    dice
}

fn perft_rec(depth: usize, position: &Position) -> u64 {
    if depth == 0 {
        return 1;
    }
    let dice = ALL_21;
    let mut count = 0;
    for die in dice {
        let children = position.all_positions_after_moving(&die);
        for child in children {
            count += match child.game_state() {
                Ongoing => perft_rec(depth - 1, position),
                GameOver(_) => 1,
            };
        }
    }
    count
}

fn perft_position(depth: usize, position: &Position, verbose: bool) -> u64 {
    let dice = ALL_21;
    let mut total = 0;
    for die in dice {
        let mut count = 0;
        let children = position.all_positions_after_moving(&die);
        for child in children {
            count += match child.game_state() {
                Ongoing => perft_rec(depth - 1, position),
                GameOver(_) => 1,
            };
        }
        if verbose {
            println!("- {}: {}", die, count);
        }
        total += count;
    }
    if verbose {
        println!("Total: {}", total);
    }
    total
}

fn perft(depth: usize, verbose: bool) -> u64 {
    perft_position(depth, &Position::new(), verbose)
}

fn main() {
    perft(2, true);
}
