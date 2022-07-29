fn main() {
    let rocket_fuel = String::from("RP-1");
    let rocket_fuel = process_fuel(rocket_fuel);
    println!("Hello, world!, {}", rocket_fuel);
}


fn process_fuel(mut propellant: String) -> String  {
    println!("processing propellant {}", propellant);

    propellant
}
