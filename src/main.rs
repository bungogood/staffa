mod backgammon;

use backgammon::{Action, State};

fn main() {
    let game = State::new();

    game.display();

    game.possible_actions((4, 5))
        .iter()
        .enumerate()
        .for_each(|(index, action)| {
            println!("{}: {}", index + 1, action);
        });

    if let Some(action) = game.action_from("24/15", (4, 5)) {
        println!("Action: {}", action);
    }
}
