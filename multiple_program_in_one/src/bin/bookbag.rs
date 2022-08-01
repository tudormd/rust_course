use multiple_program_in_one::TheBag;

fn main(){
    println!("Program: Book Bag\n");

    let mut book = TheBag::new();

    let books = vec![
        "One".to_string(),
        "Two".to_string(),
        "Three".to_string(),
    ];

    for item in books {
        book.put_in_bag(item)
    }
}
