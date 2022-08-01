fn main() {
    println!("{}", is_palindrome("Hello, world!"));
}

fn is_palindrome(value: &str)-> bool{
    let mut result = true;

    let chars: Vec<char>= value.chars().collect();

    for (index, rev_item) in value.chars().rev().enumerate() {
        if chars[index] != rev_item {
            result = false;
            break
        }
    }

    result

}
