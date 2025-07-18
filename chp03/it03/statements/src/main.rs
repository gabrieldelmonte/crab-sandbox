fn main() {
    let x = 5;
    println!("The value of x is: {}", x);

    let y = {
        let z = 3;
        z + 1
    };
    println!("The value of y is: {}", y);
}
