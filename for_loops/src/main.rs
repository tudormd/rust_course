fn main() {
    let messages = ["h","e", "l", "l", "o"];

    for (index, item) in messages.iter().enumerate() {
        println!("index is {}, item {}", index,  item);
    };

    for number in 0..5 {
        println!("{}", number);
    }
}

