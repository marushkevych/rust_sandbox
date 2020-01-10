// Vectors are resizable arrays

use std::mem;

pub fn run() {
  let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

  // Re-assign value
  numbers[2] = 20;

  println!("{:?}", numbers);
  
  // Get single value
  println!("First value: {}", numbers[2]);
  
  // Get length
  println!("Vector length: {}", numbers.len());

  // Vectors are stack allocated
  println!("Vector accupies {} bytes", mem::size_of_val(&numbers));

  // Get Slice
  let slice: &[i32] = &numbers[0..2];
  println!("Slice: {:?}", slice);

  // Add to Vector
  numbers.push(6);
  numbers.push(7);

  // Pop off last value
  numbers.pop();

  println!("{:?}", numbers);
  
  // Loop through Vector values
  for x in numbers.iter() {
    println!("Number: {}", x);
  }
  
  // Loop and mutate values
  for x in numbers.iter_mut() {
    *x *= 2;
  }
  println!("{:?}", numbers);
} 