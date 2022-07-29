use std::fs;
use std::fs::File;

fn main() {
    let contents = fs::read_to_string("the_ultimate_question.txt").expect("Nobody know the ultimate the question");
    println!("Hello, world!, {:?}", contents);
}
