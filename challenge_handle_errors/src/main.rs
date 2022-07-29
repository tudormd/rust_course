use std::io;
use std::ops::Range;
use rand::prelude::*;

fn main() {
    let secret_number = rand::thread_rng().gen_range::<u32, Range<u32>>(1..101);

    println!("I'm thinking of a number between 1 and 100..");
    println!("Guess the number: {secret_number}");

    loop {
        let mut buffer = String::new();

        // let guess = match io::stdin().read_line(&mut buffer) {
        //     Ok(_) => match buffer.trim().parse::<u32>(){
        //         Ok(value)=> value,
        //         Err(_) => {
        //             println!("Failed to parse input, Guess again");
        //             continue
        //         }
        //     },
        //     Err(_) => {
        //         println!("Failed to read input. Guest again");
        //         continue
        //     }
        // };

        let result = io::stdin().read_line(&mut buffer);

        let guess = match result {
                Ok(_) => match buffer.trim().parse::<u32>(){
                    Ok(value)=> value,
                    Err(_) => {
                        println!("Failed to parse input, Guess again");
                        continue
                    }
                },
                Err(_) => {
                    println!("Failed to read input. Guest again");
                    continue
                }
        };





        if guess > secret_number {
            println!("\n {} is too high! Guess lower:", guess);
        } else if guess < secret_number {
            println!("\n{} is too low Guess higher:", guess);
        } else {
            println!("\n You got it! The secret number was {}", secret_number);
        }
    }
}
