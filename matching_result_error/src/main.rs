use std::{fs, io};
use std::fs::File;

fn main() {
    let result = fs::read_to_string("wthe_ultimate_question.txt");
    let contents = match result {
        Ok(message) => message,
        Err(e) => match e.kind(){
            io::ErrorKind::NotFound => String::from("Not found file"),
            io::ErrorKind::PermissionDenied => String::from("Permission denied"),
            _ => panic!("Another type of error")
        }

    };
    println!("Hello, world!, {:?}", contents);
}
