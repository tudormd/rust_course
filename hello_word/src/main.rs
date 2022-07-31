fn main() {
    crate::hello::english(); // hello
    crate::hello::spanish(); // hola
    hello::spanish();
    crate::hello::casual::english() // hey
}

mod hello {
    pub fn english(){
        println!("hello");
        spanish();
        casual::english();
    }

    pub fn spanish(){
        println!("hola"); // spanish
    }

    pub mod casual {
        pub fn english(){
            println!("hey");
            // crate::hello::spanish();
            super::spanish();
        }
    }
}
