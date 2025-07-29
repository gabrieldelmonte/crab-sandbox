fn first_word_slice(some_string: &String) -> (String, usize) {
    let mut first_word = String::new();
    let bytes = some_string.as_bytes();

    for &item in bytes.iter() {
        if item == b' ' {
            break;
        }
        first_word.push(item as char);
    }

    (first_word.clone(), first_word.len())
}


fn main() {
    let str = String::from("Testing the first word slice function");

    let (word, length) = first_word_slice(&str);
    println!("Original string: '{}'", str);
    println!("First word of the string: '{}' | Length: {}", word, length);
    println!();

    let str2 = String::from("Hello world");
    let hello_string = &str2[0..5];
    let world_string = &str2[6..11];
    println!("First part: '{}', Second part: '{}'", hello_string, world_string);
    println!();

    let str3 = String::from("Another example with a longer string");
    let first_slice = &str3[0..7];
    let same_first_slice = &str3[..7];
    let second_slice = &str3[8..];
    let same_second_slice = &str3[8..];
    let exactly_same_second_slice = &str3[8..str3.len()];
    println!("Original string: '{}'", str3);
    println!("First slice: '{}'", first_slice);
    println!("Same first slice: '{}'", same_first_slice);
    println!("Second slice: '{}'", second_slice);
    println!("Same second slice: '{}'", same_second_slice);
    println!("Exactly same second slice: '{}'", exactly_same_second_slice);
    println!();
}
