use std::{fs, io};
use std::collections::HashMap;

fn main() {
    read_text_file();
}

fn read_text_file(){
    let mut buffer = String::new();

    io::stdin().read_line(&mut buffer).expect("Failed to read input line");
    let file_name = buffer.trim();

    let result = fs::read_to_string(file_name);

    let contents = match result {
        Ok(message) => message,
        Err(e) => match e.kind(){
            io::ErrorKind::NotFound => {
            eprintln!("Not found file");
            std::process::exit(1)
            },
            io::ErrorKind::PermissionDenied =>  {
                eprintln!("Permission denied");
                std::process::exit(1)
            },
            _ => panic!("Another type of error")
        }

    };

    let all_word = contents.split_whitespace().collect::<Vec<&str>>();
    let mut words_counts = HashMap::new();

    for word in all_word.iter() {
        *words_counts.entry(word).or_insert(0)+=1;
    };


    let mut top_count = 0u32;
    let mut top_words :Vec<&str> = Vec::new();


    for (&key, &val) in words_counts.iter() {
        if val > top_count {
            top_count = val;
            top_words.clear();
            top_words.push(key)
        }else if val == top_count{
            top_words.push(key)
        }
    }

    for top_word in top_words.iter() {
        println!("{}", top_word);
    }

    println!("{:?}", words_counts);
}
