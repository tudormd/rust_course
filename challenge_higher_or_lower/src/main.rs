use rand::prelude::*;
use std::io;


fn main() {
    guess_with_loop();
    // let guess_number: i32 = guess_number();
    //
    // if guess_number == 10 {
    //     println!("Correct");
    // } else if guess_number < 10 {
    //     println!("Too low");
    //     main()
    // } else {
    //     println!("Too high");
    //     main()
    // }


}

fn guess_number()-> i32{
    let number = thread_rng().gen_range(1..101);
    let mut buffer = String::new();

    println!("Guess the secret number");
    io::stdin().read_line(&mut buffer);

    let guess_number: i32 = buffer.trim().parse().unwrap();

    guess_number
}


fn guess_with_loop(){
    let secret_number = thread_rng().gen_range(1..101);

    println!("'m thinking of a number between 1 and 100 ..");
    println!("Guess the secret number");


    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("Failed to read input line");
        let guess_number: i32 = buffer.trim().parse().expect("Failed to parse the guess");

        if guess_number == 10 {
            println!("Correct");
            break;
        } else if guess_number < 10 {
            println!("Too low {}", guess_number);
        } else {
            println!("Too high {}", guess_number);
        }
    }
}
