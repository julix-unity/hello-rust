fn main() {
    println!("Hello, World!");

    let hello: Vec<i32> = (0..10).collect();

    fn do_stuff(val: &Vec<i32>) {
        println!("{}", val.len()); // this is a MACRO!
    }

    do_stuff(&hello);
}
