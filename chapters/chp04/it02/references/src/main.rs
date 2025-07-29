fn change_string(some_string: &mut String) {
    some_string.push_str(" - modified");
    println!("Changed string to: {}", some_string);
}

fn calculate_length(some_string: &String) -> usize {
    some_string.len()
}

fn main() {
    let mut first_string = String::from("Hello, world!");
    let length = calculate_length(&first_string);

    println!("The length of '{}' is {}!", first_string, length);
    println!();

    println!("Before function call: {}", first_string);
    change_string(&mut first_string);
    println!("After function call: {}", first_string);
    println!();

    let mut second_string = String::from("Rust programming");
    let ref1 = &second_string;
    let ref2 = &second_string;
    println!("First reference: {} | Second reference: {}", ref1, ref2);
    println!();

    let ref3 = &mut second_string;
    println!("Mutable reference: {}", ref3);
    ref3.push_str(" is fun!");
    println!("After modification: {}", ref3);
    println!("Final string: {}", second_string);

    // The following line would cause a compile error
    // because we cannot have a mutable reference while immutable references exist
    // println!("First reference after mutable borrow: {}", ref1);
}
