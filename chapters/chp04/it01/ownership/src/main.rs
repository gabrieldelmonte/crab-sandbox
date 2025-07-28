fn main() {
    let first = "The first variable";
    let mut second = String::from(first);
    second.push_str(" is now mutable!");

    // first = "This will not compile"; // Uncommenting this line will cause a compilation error
    println!("First: {}", first);
    println!("Second: {}", second);
    println!();

    let third = String::from("The third variable is also mutable!");
    let mut fourth = third;

    // println!("Third: {}", third); // This will cause a compilation error
    println!("Fourth: {}", fourth);

    fourth = String::from("The fourth variable has been changed!");
    println!("Fourth after change: {}", fourth);

    fourth = String::from("The fourth variable changed again!");
    println!("Fourth after another change: {}", fourth);
    println!();

    let fifth = String::from("This is the fifth variable...");
    let sixth = fifth.clone();

    println!("Fifth: {}", fifth);
    println!("Sixth: {}", sixth);
}
