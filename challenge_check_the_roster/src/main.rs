use std::env;
use std::fs;

fn main() {
    if env::args().len() < 2 {
        println!("Program require at least 2 arguments");
        std::process::exit(1);
    }

    let arg1 = env::args().nth(1).unwrap();
    let arg2 = env::args().nth(2).unwrap();
    let contents = fs::read_to_string(arg1).unwrap();

    for line in contents.lines() {
        if  line == arg2 {
            println!("Name was found {line}");
            return;
        }
    }

    println!("Name not found, {arg2}");
}
