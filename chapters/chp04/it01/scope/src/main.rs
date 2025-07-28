fn gives_ownership() -> String {
    let some_string = String::from("Hello, world!");
    println!("Inside gives_ownership: {}", some_string);

    some_string
}

fn takes_and_gives_back(mut another_string: String) -> String {
    println!("Inside takes_and_gives_back: {}", another_string);
    another_string.push_str(" - modified");

    another_string
}

fn main() {
    let first = gives_ownership();
    println!("Back in main: {}", first);
    println!();
    
    let second = String::from("Second string!");
    println!("Second string: {}", second);
    println!();
    
    let third = takes_and_gives_back(second);
    println!("Back in main after takes_and_gives_back: {}", third);
}
