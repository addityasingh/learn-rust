enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    println!("Hello, world!");
    let six = plus_one(Some(5));
    println!("The value of 5 + 1 is {} ", six);
    println!("The value of Dime is {} ", find_coin_value(Coin::Dime));
    println!("The value of Penny is {} ", find_coin_value(Coin::Penny));
    println!("The value of Nickel is {} ", find_coin_value(Coin::Nickel));
    println!("The value of quarter is {} ", find_coin_value(Coin::Quarter));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn hundered_plus_one () {
        assert_eq!(plus_one(Some(100)), 101);
    }

    #[test]
    fn null_plus_one () {
        assert_eq!(plus_one(None), 1);
    }
    
}