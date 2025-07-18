fn another_function(mut number: i32) -> i32 {
    println!("\nThe number is: {}", number);
    // number = number * 2;
    number *= 2;
    println!("Now, the number is: {}", number);
    println!("And will return multiplied by 2!");

    return number * 2;
}

fn another_another_function(number: i32, number2: i32) -> i32 {
    println!("\nThis function will return the sum of two numbers!");
    println!("The first number is: {}", number);
    println!("The second number is: {}", number2);

    return number + number2;
}

fn main() {
    println!("By now, it is the main function!");

    // Call the function with a parameter
    println!("\nCalling another_function with 5 as an argument...");
    let mut result = another_function(5);
    println!("\nThe result is: {}", result);

    // Call the another_another_function with two parameters
    println!("\nCalling another_another_function with 10 and 20 as arguments...");
    result = another_another_function(10, 20);
    println!("\nThe result is: {}", result);
}
