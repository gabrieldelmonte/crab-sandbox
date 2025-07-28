fn calculate_length(some_string: String) -> (String, usize) {
    let string_length = some_string.len();

    (some_string, string_length)
}

fn main() {
    let first = String::from("First string");
    println!("Inside main, before calling calculate_length: {}", first);
    println!();

    let (returned_string, length) = calculate_length(first);
    // println!("Inside main, after calling calculate_length: {}", first); // This line would cause an error if uncommented
    println!("Inside main, after calling calculate_length: {}", returned_string);
    println!("The length of '{}' is {}.", returned_string, length);
}

/*
fn calculate_length(some_string: &String) -> (&String, usize) {
    let string_length = some_string.len();

    (some_string, string_length)
}

fn main() {
    let first = String::from("First string");
    println!("Inside main, before calling calculate_length: {}", first);

    let (returned_string, length) = calculate_length(&first);
    println!("Inside main, after calling calculate_length: {}", first);
    println!("Inside main, after calling calculate_length: {}", returned_string);
    println!("The length of '{}' is {}.", returned_string, length);
}
*/
