/*
Only two floating point types are available in Rust:
- f32: 32-bit floating point number
- f64: 64-bit floating point number
*/

fn main() {
    let x: f32 = 3.14;              // 32-bit floating point number
    let y: f64 = 2.718281828459045; // 64-bit floating point number
    let default_float = 1.0;        // Default is f64

    println!("Floating point numbers:");
    println!("\t - f32: {x}, \n\
              \t - f64: {y}, \n\
              \t - default (f64): {default_float}" 
            );
}
