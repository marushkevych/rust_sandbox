

pub fn run() {
  greeting("Hello", "Jane");
  println!("Sum: {}", add(3, 5)) ;

  // Closure
  let n3 = 10;
  let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
  println!("Closure sum: {}", add_nums(4, 5));
}

fn greeting(greet: &str, name: &str) {
  println!("{} {}, nice to meet you", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
  n1 + n2
} 