use std::io::{Write};

fn main() {
    let mut input = String::new();
    let rows : i32;

    print!("Please, input the number of rows for the triangle: ");
    std::io::stdout()
        .flush()
        .unwrap();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");
    rows = input.trim().parse().expect("Please enter a valid number!");

    for index in 0..rows {
        for _ in 0..(rows - index - 1) {
            print!(" ");
        }
        for _ in 0..(2 * index + 1) {
            print!("*");
        }
        println!();
    }
}
