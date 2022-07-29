struct Satellite {
    name: String,
    velocity: f64 // mile per second
}

struct SpaceStation {
    name: String,
    crew_size: u8,
    altitude: u32 // miles
}

trait Description {
    fn describe(&self) -> String {
        String::from("an object flying through space")
    }
}

impl Description for Satellite {
}

impl Description for SpaceStation {
    fn describe(&self) -> String {
        format!("the {} flying {} miles high with {} crew members aboard!", self.name, self.altitude, self.crew_size)
    }
}

fn main() {
    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72
    };

    let iss  = SpaceStation {
        name: String::from("Internation Space Station"),
        crew_size: 6,
        altitude: 264
    };

    println!("hubble {}", hubble.describe());
    println!("iss {}", iss.describe());
}
