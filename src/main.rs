use guessing_game::guessing_game;

pub mod guessing_game;

use median_and_mode::{get_median, get_mode};

pub mod median_and_mode;


const RUN_GUESSING_GAME: bool = false;

fn main() {
    if RUN_GUESSING_GAME {
        guessing_game();
    }

    let vector = vec![1,2,3,4,2, 5, 5, 5,];
    let median = get_median(vector.clone());
    let mode = get_mode(vector);
    println!("Median: {median} and Mode: {:?}", {mode});
}
