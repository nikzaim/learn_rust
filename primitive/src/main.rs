// Primitive data types(Scalar Data Types)
// int, float, boolean, char

// Integer
// <--------------------------------------> //
// Signed (+ and -)
// i8, i16, i32, i64, i128(they has different range)
// <--------------------------------------> //
// Unsigned (only +)
// u8, u16, u32, u64, u128

fn main() {
    let x: i32 = -32;
    let y: u32 = 32;
    println!("Signed Integer: {}", x);
    println!("Unsigned Integer: {}", y);

    // 1 byte == 8 bits
    // i32(32 bits) and i64(64 bits)
    // i32 - 2_147_483_647
    // i64 - 9_223_372_036_854_775_807
    let i: i32 = 2_147_483_647;
    let j: i64 = 9_223_372_036_854_775_807;
    println!("Maximum value of i32: {}", i);
    println!("Maximum value of i64: {}", j);
    
    // <--------------------------------------> //
    
    // Floats [Floating Point Types]
    // f32, f64
    let pi: f64 = 3.14;
    println!("Value of pi: {}", pi);

    // <--------------------------------------> //

    //Boolean: true, false
    let is_snowing: bool = true;
    println!("Is it snowing? {}", is_snowing);

    // <--------------------------------------> //

    //Character Types - char
    let letter: char = 'a';
    println!("First letter of the alphabet: {}", letter);
}



