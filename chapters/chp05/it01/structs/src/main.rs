#[derive(Debug)]
struct User {
    username: String,
    password: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Dummy;

fn create_user(username: String, password: String, email: String) -> User {
    println!("Inside the create_user function!");

    User {
        username,
        password,
        email,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    println!("Creating two users directly in main!");
    let first_user = User {
        username: String::from("Gabriel"),
        password: String::from("password123"),
        email: String::from("gabriel@example.com"),
        active: false,
        sign_in_count: 16,
    };
    println!("First user created: {:#?}", first_user);
    println!();

    // Uncommenting the line below will cause a compile-time error because `first_user` is immutable
    // first_user.username = String::from("Gabriel2");

    let mut second_user = User {
        username: String::from("Gabrielle"),
        password: String::from("password123"),
        email: String::from("gabrielle@example.com"),
        active: true,
        sign_in_count: 1,
    };
    println!("Second user created: {:#?}", second_user);
    println!();

    println!("Changing all the fields!");
    second_user.username = String::from("Gabrielle2");
    second_user.email = String::from("gabrielle_new@example.com");
    second_user.password = String::from("newpassword123");
    second_user.active = false;
    second_user.sign_in_count += 1;
    println!("Second user after changes: {:#?}", second_user);
    println!();

    println!("Creating a new user using the create_user function!");
    let third_user = create_user(
        String::from("Gabriel"),
        String::from("password123"),
        String::from("testing@example.com"),
    );
    println!("Third user created: {:#?}", third_user);
    println!();

    // Using the struct update syntax to copy fields from `third_user`
    println!("Creating another user in the main function!");
    let fourth_user = User {
        username: String::from("Gabriel2"),
        sign_in_count: 2,
        ..third_user
    };
    println!("Fourth user created: {:#?}", fourth_user);
    println!();

    println!("Creating a tuple struct for color!");
    let first_color = Color(0, 0, 0);
    println!("Black color created: {:#?}", first_color);
    println!("Accessing the first element of the tuple struct: {}", first_color.0);
    println!("Accessing the second element of the tuple struct: {}", first_color.1);
    println!("Accessing the third element of the tuple struct: {}", first_color.2);
    println!();

    println!("Creating a unit struct!");
    let unit_struct = Dummy;
    println!("Unit struct created: {:#?}", unit_struct);
}
