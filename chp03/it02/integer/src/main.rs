/*
Length  | Signed    | Unsigned
--------|-----------|----------
8-bit   | i8        | u8
16-bit  | i16       | u16
32-bit  | i32       | u32
64-bit  | i64       | u64
128-bit | i128      | u128
arch    | isize     | usize
*/

/*
Number literals | Example
----------------|--------
Decimal         | 98_222
Hexadecimal     | 0xff
Octal           | 0o77
Binary          | 0b1111_0000
Byte (u8)       | b'A'
*/

fn main() {
    // Signed integers
    let signed_8: i8 = -128;                                            // Minimum value for i8
    let signed_16: i16 = -32768;                                        // Minimum value for i16
    let signed_32: i32 = -2147483648;                                   // Minimum value for i32
    let signed_64: i64 = -9223372036854775808;                          // Minimum value for i64
    let signed_128: i128 = -170141183460469231731687303715884105728;    // Minimum value for i128
    let signed_arch: isize = -9223372036854775808;                      // Assuming 64-bit architecture

    // Unsigned integers
    let unsigned_8: u8 = 255;                                           // Maximum value for u8
    let unsigned_16: u16 = 65535;                                       // Maximum value for u16
    let unsigned_32: u32 = 4294967295;                                  // Maximum value for u32
    let unsigned_64: u64 = 18446744073709551615;                        // Maximum value for u64
    let unsigned_128: u128 = 340282366920938463463374607431768211455;   // Maximum value for u128
    let unsigned_arch: usize = 18446744073709551615;                    // Assuming 64-bit architecture

    println!("Signed integers:");
    println!("\t - i8: {signed_8},       \n\
              \t - i16: {signed_16},     \n\
              \t - i32: {signed_32},     \n\
              \t - i64: {signed_64},     \n\
              \t - i128: {signed_128},   \n\
              \t - isize: {signed_arch}" 
            );

    println!("\n// ------------------------------------ //\n"); 

    println!("Unsigned integers:");
    println!("\t - u8: {unsigned_8},       \n\
              \t - u16: {unsigned_16},     \n\
              \t - u32: {unsigned_32},     \n\
              \t - u64: {unsigned_64},     \n\
              \t - u128: {unsigned_128},   \n\
              \t - usize: {unsigned_arch}" 
            );
}
