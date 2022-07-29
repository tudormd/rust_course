use std::fmt;
fn compare_and_print<T, U>(a: T, b: U)
   where T: fmt::Display+PartialEq+From<U>,
         U: fmt::Display+PartialEq+Copy
{
//fn compare_and_print<T: fmt::Display+PartialEq+From<U>, U: fmt::Display+PartialEq+Copy>(a: T, b: U) {
    if a == T::from(b){
        println!("{} is equal to {}", a, b);
    } else {
        println!("{} is Not equal to {}", a, b);
    }

}

fn main() {
    compare_and_print(1.0,1);
    compare_and_print(1.1,1);
    compare_and_print(1.1,"1111");
}
