pub fn borrow (s: &String) {
    println!("value of s {} can be reused now", s);
}

pub fn borrow_with_mutate(s: &mut String) {
    s.push_str(", World!");
}