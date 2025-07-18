fn main() {
    let condition = true;
    let number_if = if condition {
        5
    }
    else {
        6
    };
    println!("The value of number_if is: {}", number_if);

    let mut input = String::new();
    let number : i32;

    println!("Please, input a number: ");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    number = input.trim().parse().expect("Please enter a valid number!");

    // Even with just one line, it is obligatory to use curly braces
    // to define the scope of the if statement.
    // Diferent from C, Rust does not allow single-line if statements
    // without braces.
    if number % 2 == 0 {
        println!("\nThe number {} is even!", number);
    }
    else {
        println!("\nThe number {} is odd!", number);
    }

    // Other thing is that Rust does not allow
    // is to use integers as boolean values.
    // if number {
    if number != 0 {
        println!("\nThe number {} is a truthy value!", number);
    }
    else {
        println!("\nThe number {} is a falsy value!", number);
    }

    // if, else if, else
    if number < 0 {
        println!("\nThe number {} is negative!", number);
    }
    else if number == 0 {
        println!("\nThe number {} is zero!", number);
    }
    else {
        println!("\nThe number {} is positive!", number);
    }
}
