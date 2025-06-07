use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please, input your guess: ");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("\nYou guessed: {}", guess);

    /*
    let guess_immutable = 5;
    let mut guess_mutable = 5;
    */
}
