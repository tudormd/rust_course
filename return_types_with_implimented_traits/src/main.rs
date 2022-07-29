use std::fmt;

fn get_display() -> impl fmt::Display {
    13
}

fn main() {
    println!("Hello, world! {}", get_display());
}



