// Conditionals - Used to check the condition of something and act on the result

pub fn run() {
  let age = 18;
  let check_id = false;
  let knows_person_of_age = true;

  // if/else
  if age >= 21 && check_id || knows_person_of_age {
    println!("Bartender: What would you like to dring?");
  } else if age < 32 && check_id {
    println!("Bartender: Sorry you have to leave");
  } else {
    println!("Bartender: I'll need to see your Id");
  }

  // used as an expression
  let is_of_age = if age >= 21 { true } else { false };

  println!("Is of age: {}", is_of_age);
}