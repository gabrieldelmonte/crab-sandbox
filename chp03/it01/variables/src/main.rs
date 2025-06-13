fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}");
}

/*
// This code will not compile because `x` is immutable by default in Rust
fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    
    x = 6;
    println!("The value of x is: {x}");
}
*/
