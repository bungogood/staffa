mod backgammon;

use backgammon::State;

fn main() {
    let game = State::from_id("zGbiIQgxH/AAWA".to_string());

    game.display();
}
