use guessing_game::guessing_game;

pub mod guessing_game;

const run_guessing_game: bool = false;

fn main() {
    if run_guessing_game {
        guessing_game();
    }

    let median = get_median(vec![1,2,3,4,2, 5, 5, 5,]);
    println!("{median}");
}

    /*
     * Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and
     */
fn get_median (mut list: Vec<i32>) -> f32 {

    // median
    list.sort();
    let length = list.len();
    let isEven = length % 2 == 0;
    
    match isEven {
        true => (list[length / 2] as f32 + list[(length / 2) - 1] as f32) / 2.0,
        false => list[length / 2] as f32
    };
}
