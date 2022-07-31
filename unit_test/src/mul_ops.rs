use std::fmt::Display;
use std::ops::Mul;

pub fn mul_two<T:Mul<Output=T> + Display>(a:T, b:T) -> T{
    print_mul(&[&a, &b]);
    a * b
}

pub fn mul_three<T:Mul<Output=T> + Display>(a:T, b:T,c : T) -> T{
    print_mul(&[&a, &b,&c]);
    a * b * c
}


fn print_mul<T: Display>(operands:&[T]) -> String {
    let mut message = String::from("Multiplying");

    if let Some((last, elements)) = operands.split_last(){
        for n in elements {
            message.push_str(&format!("{}", n));
        }
        message.push_str(&format!("{}", last))
    }

    println!("{}", message);
    message
}
