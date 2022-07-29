use std::fs;
use std::io::prelude::*;

fn main() {
    let mut speech = String::new();
    speech.push_str("We choose to the Moon in this decade\n");
    speech.push_str("and do to other things, \n");
    speech.push_str("not because they are easy, \n");
    speech.push_str("but because they are hard, \n");

    fs::write("speech.txt", speech);

    let mut file  = fs::OpenOptions::new().append(true).open("planetsc.txt").unwrap();

    file.write(b"\nPluto");
}

