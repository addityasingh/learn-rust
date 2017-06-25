enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    println!("Hello, world!");
    let six = plus_one(Some(5));

    println!("The value of Dime is {} ", find_coin_value(Coin::Dime));
}

fn find_coin_value(coin: Coin) -> i32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(n: Option<i32>) -> i32 {
    match n {
        None    => 1,
        Some(x) => x + 1,
    }
}