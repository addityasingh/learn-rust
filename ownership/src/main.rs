mod borrow;
mod ownership;
mod slices;

fn main() {
    
    let s = String::from("hello");  // s comes into scope.

    borrow::borrow(&s);                     // s is just borrowed and can still be reused 

    ownership::takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here.
    
    // The below line will fail at compile time
    // println!("value of s is {}", s);

    let x = 5;                      // x comes into scope.

    ownership::makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward.

    println!("value of x is {}", x);

    // borrow with mutate is not allowed by default ...
    // but we can achieve mutation on borrowed variable ... 
    // using mutable type
    // borrow_with_mutate(&s);
    let mut s2 = String::from("hello");
                                    
    let r1 = &mut s2;
    borrow::borrow_with_mutate(r1);

    println!("value of s2 is {} ", r1);

    let word = slices::first_word(r1);
    println!("first word is {}", word);
    r1.clear();
}