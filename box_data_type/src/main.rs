use std::mem;
// Store data on the heap instead of on the stack

struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64,
}


fn main() {
    let mut vehicle = Shuttle {
        name: String::from("Endeavour"),
        crew_size: 7,
        propellant: 0.0
    };

    println!("stack! {}", mem::size_of_val(&vehicle));

    let boxed_vehicle : Box<Shuttle>  = Box::new(vehicle);

    println!("boxed stack! {}", mem::size_of_val(&boxed_vehicle));
    println!("boxed heap! {}", mem::size_of_val(&*boxed_vehicle));

    let unboxed_vehicle: Shuttle = *boxed_vehicle;
    println!("unboxed_vehicle stack! {}", mem::size_of_val(&unboxed_vehicle));


}
