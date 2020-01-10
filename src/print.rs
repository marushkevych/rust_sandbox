pub fn run() {
  // Print to console
  println!("Hello from the print.rs");

  // basic formatting
  println!("{} is from {}", "Brad", "Mass");
  
  // positional arguments
  println!("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "code");
  
  // named arguments
  println!("{name} likes to play {activity}", name = "John", activity = "Baseball");

  // PLaceholder traits
  println!("Bunary: {:b} Hex: {:x} Octal: {:0}", 10, 10, 10);

  // Placeholder for debug trait
  println!("{:?}", (12, true, "hello"));

  // Basic math
  println!("10 + 10 = {}", 10 + 10);
}
