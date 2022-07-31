use rand::prelude::*;
// use rand::{Rng, thread_rng};
// use rand::*;

fn main() {
    let mut rng   = thread_rng();
    let  num = rng.gen::<u32>();
    println!("Hello, world!, {}", num);
}
