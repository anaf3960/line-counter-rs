use std::io::{self, BufRead};

fn main() {
    let count = io::stdin().lock().lines().count();
    println!("Lines: {}", count);
}
