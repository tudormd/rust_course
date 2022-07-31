// use crate::greeting::formal;
// use crate::greeting::casual;

use crate::greeting::{formal,casual};

fn main() {
    greeting::formal::english(); // hello
    formal::spanish(); // hola
    casual::spanish(); // hola
}

mod greeting {
    pub mod formal{
        pub fn english(){
            println!("hello");
        }

        pub fn spanish(){
            println!("hola"); // spanish
        }
    }

    pub mod casual {
        pub fn english(){
            println!("hey");
        }

        pub fn spanish(){
            println!("hola"); // spanish
        }
    }
}
