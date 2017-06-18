fn main() {
    
    let s = String::from("hello");  // s comes into scope.

    borrow(&s);                     // s is just borrowed and can still be reused 

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here.
    
    // The below line will fail at compile time
    // println!("value of s is {}", s);

    let x = 5;                      // x comes into scope.

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward.

    println!("value of x is {}", x);

    // borrow with mutate is not allowed by default ...
    // but we can achieve mutation on borrowed variable ... 
    // using mutable type
    // borrow_with_mutate(&s);
    let mut s2 = String::from("hello");
                                    
    let r1 = &mut s2;
    borrow_with_mutate(r1);

    print!("value of s2 is {} ", r1);
}

fn takes_ownership(some_string: String) { // some_string comes into scope.
    println!("owned variable {}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope.
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn borrow (s: &String) {
    println!("value of s {} can be reused now", s);
}

fn borrow_with_mutate(s: &mut String) {
    s.push_str(", World!");
}