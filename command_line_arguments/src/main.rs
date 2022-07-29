use std::env;

fn main() {
    if env::args().len() <= 2 {
        println!("Program require at least 2 arguments");
    }

    for (index, arguments) in env::args().enumerate() {
        println!("index {} and arguments {}", index, arguments);

    }

    let arg2 = env::args().nth(2).unwrap();
    println!("{}", arg2);
}
