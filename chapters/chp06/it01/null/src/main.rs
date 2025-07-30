fn main() {
    // Option<i32>
    println!("Initializing a number with \"Some\" prefix");
    let some_number = Some(42);
    println!("The number is: {:#?}", some_number);
    println!();

    // Option<char>
    println!("Initializing a char with \"Some\" prefix");
    let some_char = Some('a');
    println!("The character is: {:#?}", some_char);
    println!();

    // Option<i32>
    println!("Initializing a number with \"None\" prefix");
    let mut absent_number: Option<i32> = None;
    println!("The absent number is: {:#?}", absent_number);
    println!();

    println!("Changing the absent number to a new value!");
    absent_number = Some(100);
    println!("The (old) absent number is: {:#?}", absent_number);
    println!();

    println!("Understanding the T type of Option<T>!");
    let x: i8 = 5;
    let y: Option<i8> = Some(10);
    println!("x is: {:#?}", x);
    println!("y is: {:#?}", y);

    println!("To add x and y, we need to handle the Option type properly.");
    // let sum = x + y; // This line would cause a compile error
    let sum = x + y.unwrap_or(0);
    println!("The sum of x and y is: {}", sum);
    println!();
}
