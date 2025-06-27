

fn main() {
    // Tuples
    let tup = (150, 25.7, 1, "Gabriel", true);
    println!("Tuple: {tup:?}");

    let (x, y, z, name, is_active) = tup;
    println!("\nTuple elements: \n\
                x = {x}, \n\
                y = {y}, \n\
                z = {z}, \n\
                name = {name}, \n\
                is_active = {is_active}");

    let a: (i32, f64, u8) = (500, 3.14, 255);
    println!("\nTuple with explicit types: {a:?}");

    let five_hundred = a.0;
    let pi = a.1;
    let max_u8 = a.2;
    println!("\nTuple elements by index: \n\
                five_hundred = {five_hundred}, \n\
                pi = {pi}, \n\
                max_u8 = {max_u8}");

    // Arrays
    let months: [&str; 12] = [
        "January", "February", "March", "April", "May", "June",
        "July", "August", "September", "October", "November", "December"
    ];
    println!("\nArray of months: {months:?}");
    println!("First month: {}", months[0]);
//    println!("Last month: {months[11]}"); // Error!
    println!("Last month: {}", months[11]); // Corrected access to last element

    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("\nMutable array before change: {numbers:?}");
    numbers[0] = 10;
    println!("Mutable array after change: {numbers:?}");

    let empty_array: [i32; 0] = [];
    println!("\nEmpty array: {empty_array:?}");

    // Slices
    let slice: &[i32] = &numbers[1..4];
    println!("\nSlice of numbers from index 1 to 3: {slice:?}");

    // --- Invalid array indexing --- //

    let vect = [1, 2, 3, 4, 5];

    println!("\nArray: {vect:?}");
    println!("Length of the array: {}", vect.len());
    println!("\nPlease enter an array index.");

    let mut index = String::new();

    std::io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = vect[index];

    println!("The value of the element at index {index} is: {element}");
}
