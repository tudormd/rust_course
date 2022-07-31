use std::fmt::{Display};
use std::ops::Add;

/* add two number of the same type */
pub fn add_two<T: Add<Output=T> + Display>(a: T, b: T) -> T {
    print_add(&[&a, &b]);
    a+b
}

/* add three number of the same type */
pub fn add_three<T: Add<Output=T> + Display>(a: T,  b: T, c: T) -> T {
    print_add(&[&a, &b, &c]);

    a + b+ c
}

/* print a message displaying operand being added */
fn print_add<T: Display>(operands: &[T]) -> String {
    let mut message = String::from("Adding ");

    if let Some((last, elements)) = operands.split_last(){
        for n in elements {
            message.push_str(&format!("{} + ", n))
        }

        message.push_str(&format!("{}", last))
    }
    println!("{}", message);
    message
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_add_two(){
        assert_eq!(add_two(1, 2), 3);
        assert_eq!(add_two(1.2, 3.4), 4.6);
    }

    #[test]
    fn ut_add_three(){
        assert_eq!(add_three(1, 2, 3), 6);
        assert_eq!(add_three(1.2, 3.4, 5.6), 10.2);
    }

    #[test]
    fn ut_print_addition(){
        let message = print_add(&['x', 'y','z']);
        assert_eq!(message, String::from("Adding x + y + z"));
    }

}
