

pub fn run() {
  let mut count = 0;
  
  // infinite loop
  loop {
    count += 1;
    println!("Count: {}", count);

    if count == 20 {
      break;
    }
  }

  // while loop 
  // (FizzBuzz)
  // If divisible by
  // 3 - print Fizz
  // 5 - print Buzz
  // both - print FizzBuzz
  let mut number = 0;
  while number <= 30 {
    if number % 15 == 0 {
      println!("fizzbuss");
    }
    else if number % 3 == 0 {
      println!("fizz");
    }
    else if number % 5 == 0 {
      println!("buzz");
    }
    else {
      println!("{}", number);
    }

    // inc
    number += 1;
  }

  // For Range
  for x in 0..30 {
    if x % 15 == 0 {
      println!("fizzbuss");
    }
    else if x % 3 == 0 {
      println!("fizz");
    }
    else if x % 5 == 0 {
      println!("buzz");
    }
    else {
      println!("{}", x);
    }
  }

}