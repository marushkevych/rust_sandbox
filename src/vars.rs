// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
  let name = "Brad";
  // mutable variable
  let mut age = 37;
  println!("My name is {} and I am {}", name, age);

  age = 66;
  println!("My name is {} and I am {}", name, age);

  // Define constant (type is mandatory for const)
  const ID: i32 = 001;
  println!("Id: {}", ID);

  // Assing multiple vars
  let (my_name, my_age) = ("Brad", 37);
  println!("{} is {}", my_name, my_age);
}