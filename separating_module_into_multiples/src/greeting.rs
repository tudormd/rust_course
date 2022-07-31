pub fn description(){
    println!("greeting message");
}

pub mod formal{
    pub fn english(){
        println!("hello");
    }
    pub fn spanish(){
        println!("hola");
    }
}

pub mod casual; // submodule in greeting/ directory
