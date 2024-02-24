pub use hello_rust::guessing_game;
pub use hello_rust::{get_median, get_mode};
pub use hello_rust::pig_latin_ify;

const RUN_GUESSING_GAME: bool = false;

fn main() {
    if RUN_GUESSING_GAME {
        guessing_game();
    }

    let vector = vec![1, 2, 3, 4, 2, 5, 5, 5,];
    let median = get_median(vector.clone());
    println!("Median: {median} and Mode: {:?}", get_mode(vector));
    println!("{}", pig_latin_ify(String::from("apple")));
}
