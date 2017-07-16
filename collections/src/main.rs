extern crate chrono;

use chrono::{DateTime, Local, Utc};

struct Stock {
    open: f64,
    close: f64,
    high: f64,
    low: f64,
    date: DateTime<Utc>
}

fn main() {
    println!("Hello, world!");

    let mut stocks: Vec<Stock> = Vec::new();
    let mut stock = Stock {
        open: 1.2, 
        close: 2.4,
        high: 3.6,
        low: 1.2,
        date: Utc::now()
    };

    stocks.push(stock);
}
