#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    let mut rectangle = Rectangle { width: 30, height: 50 };
    println!("Created rectangle: {:#?}", rectangle);
    println!("The area of the rectangle is {} square pixels.", area(&rectangle));
    println!();

    println!("Changing the rectangle dimensions...");
    rectangle.width = 10;
    rectangle.height = 20;
    println!("Updated rectangle: {:#?}", rectangle);
    println!("The area of the updated rectangle is {} square pixels.", area(&rectangle));
    println!();
}
