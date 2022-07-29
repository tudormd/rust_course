fn main() {
    let outer_planet: String;
    {
        let mut inner_planet = String::from("Mercury");
        outer_planet = inner_planet.clone();
        inner_planet.clear();
        println!("Hello, world!, {}",  inner_planet);
    }
    println!("Hello, world!, {}",  outer_planet);
}

