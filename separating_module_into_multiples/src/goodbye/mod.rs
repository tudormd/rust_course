pub fn description(){
    println!("goodbye message");
}

pub mod formal { // inline submodule
    pub fn english(){
    println!("goodbye");
    }
    pub fn spanish(){
        println!("adios");
    }
}

pub mod casual; // submodule in same directory
