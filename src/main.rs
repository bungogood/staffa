mod backgammon;

use backgammon::{Action, State};

fn main() {
    let game = State::new();
    let action = Action::from("6/3").unwrap();

    println!("{}", action);

    game.display();

    game.apply_action(action).display();
}
