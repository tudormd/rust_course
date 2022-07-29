fn main() {
    let my_number = 4u8;

    let result = match my_number {
        0 => "zero",
        1 => "one",
        2 => "two",
        3 => "three",
        _ => {
            println!(" {} Did not match", my_number);
            "something else"
        }
    };

    println!("Hello, world!, {}", result);
}
