mod backgammon;

use backgammon::State;

fn main() {
    let game = State::new();

    game.display();
}
