
fn get_biggest<T: PartialOrd>(a: T, b: T) -> T {
     if a > b {
         a
     } else {
         b
     }
}

fn main() {
    println!("Hello, world!, {}", get_biggest(1.2, 2.2));
}


