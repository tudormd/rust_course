use std::io;

fn main() {
    let mut buffer = String::new();
    println!("Enter a message");
    io::stdin().read_line(&mut buffer);

    println!("buffer is {}", buffer);

    let number: i32 = buffer.trim().parse().unwrap();
    // let number = buffer.trim().parse::<i32>();
    // let number: i32 = buffer.trim().parse();

    println!("number is {}", number);


}
