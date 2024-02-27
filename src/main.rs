pub use hello_rust::guessing_game;
pub use hello_rust::{get_median, get_mode};
pub use hello_rust::pig_latin_ify;
pub use hello_rust::department_employees;

const RUN_GUESSING_GAME: bool = false;
const RUN_DEPARTMENT_EMPLOYEES: bool = true;



fn main() {
    if RUN_GUESSING_GAME {
        guessing_game();
    }    
    if RUN_DEPARTMENT_EMPLOYEES {
        department_employees();
    }

    let vector = vec![1, 2, 3, 4, 2, 5, 5, 5,];
    let median = get_median(vector.clone());
    println!("Median: {median} and Mode: {:?}", get_mode(vector));
    println!("{}", pig_latin_ify(String::from("apple")));
}
