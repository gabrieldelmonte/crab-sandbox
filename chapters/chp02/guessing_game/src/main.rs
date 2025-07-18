// Quiting
use rand::Rng;

fn main() {
    println!("\nGuess the number between 1 and 100!");
    println!("Type 'quit' to exit the game at any time.\n");
    let secret_number = rand::rng().random_range(1..=100); // The generator is inclusive of both ends: [1, 100]

    loop {
        let mut guess = String::new();
        
        println!("Please, input your guess: ");
        std::io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

        if guess.trim() == "quit" {
            println!("\nYou choose to quit the game. Goodbye!");
            break;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input! Please enter a valid number or type 'quit' to exit.");
                continue;
            }
        };

        println!("");
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You guessed it!");
                break;
            }
        }
    }

}


/*
// Loop
use rand::Rng;

fn main() {
    println!("\nGuess the number between 1 and 100!");
    let secret_number = rand::rng().random_range(1..=100); // The generator is inclusive of both ends: [1, 100]
    
    loop {
        let mut guess = String::new();
        
        println!("Please, input your guess: ");
        std::io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    let guess: u32 = guess.trim().parse().expect("Please enter a number!");
    
    println!("");
    match guess.cmp(&secret_number) {
        std::cmp::Ordering::Less => println!("Too small!"),
        std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You guessed it!");
                break;
            }
        }
    }
    
}
*/

/*
// Match
use rand::Rng;

fn main() {
    println!("\nGuess the number between 1 and 100!");

    let secret_number = rand::rng().random_range(1..=100); // The generator is inclusive of both ends: [1, 100]

    println!("Please, input your guess: ");
    let mut guess = String::new();
    std::io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please enter a number!");

    println!("");
    match guess.cmp(&secret_number) {
        std::cmp::Ordering::Less => println!("Too small!"),
        std::cmp::Ordering::Greater => println!("Too big!"),
        std::cmp::Ordering::Equal => println!("You guessed it!"),
    }

    println!("You guessed: {guess} and the secret number is: {secret_number}");

}
*/

/*
// Rand
use rand::Rng;

fn main() {
    println!("Guess the number between 1 and 100!");

    //    let secret_number = rand::thread_rng().gen_range(1..=100);
    let secret_number = rand::rng().random_range(1..=100); // The generator is inclusive of both ends: [1, 100]

    let mut guess = String::new();
    println!("Please, input your guess: ");
    std::io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");

//    println!("\nYou guessed: {} and the secret number is: {}", guess, secret_number);

println!("\nYou guessed: {guess} and the secret number is: {secret_number}");

}
*/

/*
// Input and Output
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
*/
