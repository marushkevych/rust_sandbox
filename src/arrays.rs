// Arrays - Fixed list where elements are the same data types
use std::mem;

pub fn run() {
  // let numbers = [1, 2, 3, 4];
  let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

  // Re-assign value
  numbers[2] = 20;

  println!("{:?}", numbers);
  
  // Get single value
  println!("First value: {}", numbers[2]);
  
  // Get length
  println!("Array length: {}", numbers.len());

  // Arrays are stack allocated
  println!("Array accupies {} bytes", mem::size_of_val(&numbers));

  // Get Slice
  let slice: &[i32] = &numbers[0..2];
  println!("Slice: {:?}", slice);

}