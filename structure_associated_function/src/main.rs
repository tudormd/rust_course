struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64,
}


impl  Shuttle {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn add_fuel(&mut self, gallons: f64){
        self.propellant+= gallons
    }

    fn new (name: &str) -> Shuttle {
         Shuttle {
             name: String::from(name),
             crew_size: 7,
             propellant: 0.0
         }
    }
}

fn main() {
    let mut vehicle  = Shuttle::new("Endeavour");
    println!("Hello, world! {} ", vehicle.name);
}
