
// run program wiht some args:
// > cargo run hello world

pub fn run() {
  // grub command line args
  let args: Vec<String> = std::env::args().collect();

  let command = &args[1];

  println!("Args: {:?}", args);
  println!("Command: {}", command);
}