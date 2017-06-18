pub fn takes_ownership(some_string: String) { // some_string comes into scope.
    println!("owned variable {}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

pub fn makes_copy(some_integer: i32) { // some_integer comes into scope.
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
