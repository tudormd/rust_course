use std::fs;

fn main() {
    let contents = fs::read_to_string("planets.txt").unwrap();

    // println!("{}", contents);

    for line in contents.lines() {
        println!("{}", line);
    }

    let contents = fs::read("planets.txt").unwrap();

    println!("{:?}", contents);
}
