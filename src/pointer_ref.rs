// Reference Pointers - Points to a resource in memory

pub fn run() {
  // primitive array
  let mut arr1 = [1,2,3];
  let arr2 = arr1;
  arr1[2] = 10; // this doesnt affect arr2

  println!("Arrays: {:?}", (arr1, arr2));

  // With non-primitives, if you assign another variable to a piece of data,
  // the first variable no longer holds that value. 
  // You'll need to use reference (&) to point to that resource

  // Vector
  let vec1 = vec![1,2,3];
  let vec2 = &vec1;

  println!("Vectors: {:?}", (&vec1, vec2));
}