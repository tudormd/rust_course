use std::env;

fn main() {
    let sum = env::args().skip(1).fold(0, |acc, x|{
        acc + x.parse::<i32>().expect("All arg must be integers")
    });

    println!("{}", sum);
}
