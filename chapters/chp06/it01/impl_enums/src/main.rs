enum Message {
    ChangeColor(i32, i32, i32),
    Move {
        x: i32,
        y: i32
    },
    Quit,
    Write(String),
}

impl Message {
    fn call(&self) {
        match self {
            Message::ChangeColor(r, g, b) => {
                println!("Changing color to red: {}, green: {}, blue: {}", r, g, b);
            }
            Message::Move { x, y } => {
                println!("Moving to position x: {}, y: {}", x, y);
            }
            Message::Quit => {
                println!("Quitting the application");
            }
            Message::Write(text) => {
                println!("Writing message: {}", text);
            }
        }
    }
}

fn main() {
    println!("Changing the color of the message!");
    let mut message = Message::ChangeColor(255, 0, 0);
    message.call();
    println!();
    
    println!("Moving the message!");
    message = Message::Move { x: 10, y: 20 };
    message.call();
    println!();
    
    println!("Writing a message!");
    message = Message::Write(String::from("Hello, world!"));
    message.call();
    println!();
    
    println!("Quitting the application!");
    message = Message::Quit;
    message.call();
    println!();
}
