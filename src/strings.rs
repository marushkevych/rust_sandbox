// Primitive &str = Immutable fixed-length string somehwere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run() {
  // primitive str
  let foo = "foo";
  println!("{}", foo);

  // String
  let mut hello = String::from("Hello");

  // push char
  hello.push(',');

  // push string
  hello.push_str(" World!");

  println!("{}", hello);

  // Some String methods
  println!("Capacity in bytes: {}", hello.capacity());
  println!("Is Empty: {}", hello.is_empty());
  println!("Length {}", hello.len());
  println!("Contains World: {}", hello.contains("World"));
  println!("Replace: {}", hello.replace("World", "there"));

  // Loop through string by whitespace
  for word in hello.split_whitespace() {
    println!("{}", word);
  } 
  
  // Create string with capacity
  let mut s = String::with_capacity(10);
  s.push('a');
  s.push('b');
  println!("{}", s);

  // Assertion testing
  assert_eq!(2, s.len());
  assert_eq!(10, s.capacity());
}