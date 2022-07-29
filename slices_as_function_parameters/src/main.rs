fn main() {
    let message = String::from("Greetings from Earth!");
    let first_word = get_first_word(&message);

    println!("Hello, world!, {}", first_word);
}


fn get_first_word(s: &String)->&str {
    let bytes = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[..index];
        }
    }
    &s
}
