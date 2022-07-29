fn main() {
    let mut rocket_fuel = String::from("RP-1");
    let length = process_fuel(&rocket_fuel);
    println!("Hello, world!, {}, {}", rocket_fuel, length);
}


fn process_fuel(mut propellant: &String)-> usize  {
    println!("processing propellant {}", propellant);
    let length = propellant.len();

    length
}


fn process_fuel_mut(mut propellant: & mut String)-> usize  {
    println!("processing propellant {}", propellant);
    propellant.push_str("is home");
    let length = propellant.len();

    length
}
