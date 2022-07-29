#[derive(Debug)]
#[derive(Clone)]
struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64,
}


fn main() {
    let mut vehicle = Shuttle {
        name: String::from("Endeavour"),
        crew_size: 7,
        propellant: 835958.0
    };

    let mut vehicle2 = Shuttle {
        // name: String::from("Discovery"), // one owner
        ..vehicle.clone()
    };

    println!("Hello, world! {}", vehicle.name);

    vehicle.name   = String::from("Atlantis");

    println!("Hello, world! {:?}", vehicle);
    println!("Hello, world! {:?}", vehicle2);

}
