fn main() {
    let x = 5;
    println!("The value of x is: {x}");

    let x = x + 1;
    println!("The value of x is: {x}");

    {
        let x = x * 2;
        println!("The value of x is: {x}");
    }

    println!("The value of x is: {x}");
}

/*
// The following code snippet will not compile because `spaces` is shadowed by a variable with different type.
fn main() {
    let mut spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {spaces}");
}
*/
