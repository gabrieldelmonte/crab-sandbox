fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        
        if counter == 5 {
            break counter * 10;
        }
    };
    
    println!("The result is {} and the counter is {}!", result, counter);

    println!("\n\n\n");

    counter = 0;
    let mut remaining;
    'counting_up: loop {
        println!("Counter: {}", counter);
        remaining = 10;

        loop {
            println!("Remaining: {}", remaining);
            if remaining == 8 {
                println!("Breaking the inner loop");
                break;
            }
            if counter == 4 {
                println!("Breaking the outer loop");
                break 'counting_up;
            }
            remaining -= 1;
        }
        counter += 1;
    }
    println!("Final counter: {}", counter);
    println!("Final remaining: {}", remaining);

    println!("\n\n\n");

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    let vector = vec!['a', 'b', 'c', 'd', 'e'];
    let mut index = 0;

    while index < vector.len() {
        println!("Current element: {} | Current index: {}", vector[index], index);
        index += 1;
    }

    for element in vector {
        println!("Element: {}", element);
    }

    println!("\n\n\n");

    for number in (1..=5).rev() {
        println!("Countdown: {}", number);
    }

    for number in (1..5) {
        println!("Counting up: {}", number);
    }
}
