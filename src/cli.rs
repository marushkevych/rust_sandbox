
// run program wiht some args:
// > cargo run hello world

pub fn run() {
  // grub command line args
  let args: Vec<String> = std::env::args().collect();

  println!("Args: {:?}", args);
  
  let command = &args.get(1);

  match command {
    Some(x) => println!("Command: {}", x),
    None => println!("Please provide command argument")
  }
}