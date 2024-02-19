use guessing_game::guessing_game;

pub mod guessing_game;

use median_and_mode::{get_median};

pub mod median_and_mode;


const run_guessing_game: bool = false;

fn main() {
    if run_guessing_game {
        guessing_game();
    }

    let median = get_median(vec![1,2,3,4,2, 5, 5, 5,]);
    println!("{median}");
}
