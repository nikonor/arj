use chrono::prelude::*;

fn main() {
    println!("Hello, world!");
    let year = Local::now().date().year();
    println!("year={}", year);
}
