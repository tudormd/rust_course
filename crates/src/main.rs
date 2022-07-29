use rand::prelude::*;
use std::io;

fn main() {
    let number = random::<f64>();
    println!("Hello, world!, {}", number);

    let number = thread_rng().gen_range(1..11);

    println!("Hello, world!, {}", number);


}
