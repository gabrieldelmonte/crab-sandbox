#[derive(Debug)]
struct Rectangle {
    pub width: u32,
    height: u32,
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Rectangle {
        if width == 0 {
            panic!("Width must be greater than zero!");
        }
        if height == 0 {
            panic!("Height must be greater than zero!");
        }

        Rectangle { width, height }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_fit(&self, other_rectangle: &Rectangle) -> bool {
        self.width >= other_rectangle.width && self.height >= other_rectangle.height
    }
}

fn main() {
    let rectangle = Rectangle::new(10, 20);
    println!("First rectangle: {:#?}", rectangle);
    println!("Area of the first rectangle: {}", rectangle.area());
    println!();

    let smaller_rectangle = Rectangle::new(5, 10);
    println!("Second rectangle: {:#?}", smaller_rectangle);
    println!("Area of the second rectangle: {}", smaller_rectangle.area());
    println!();

    println!("Can the first rectangle fit the second? {}", rectangle.can_fit(&smaller_rectangle));
    println!();
}
