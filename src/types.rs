/*
Primitive Types
---------------
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
Floats: f32, f64
Boolean: bool
Character: char
Tuples
Arrays
*/

pub fn run() {
 // Find max size
 println!("Max i32: {}", std::i32::MAX);
 println!("Max i64: {}", std::i64::MAX);

 // char is a unicode (you can use emoji)
 let face = '\u{1F600}';
  println!("Smile: {}", face);
}