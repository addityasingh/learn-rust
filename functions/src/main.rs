fn main() {
    println!("Hello, world!");

    for num in 1..6 {
        let f = fact(num);
        println!("factorial of {} is {}", num, f);
    }
}

/// Function to find factorial of a number
/// using if-else as expression
fn fact2(n: i32) -> i32 {
    if n == 1 { n } else { n * fact(n - 1) }
}

/// Function to find factorial of a number
/// using pattern matching
fn fact(n: i32) -> i32 {
    match n {
        0 => 1,
        _ => n * fact(n - 1),
    }
}
