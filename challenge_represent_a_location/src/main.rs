enum Location {
    Unknown,
    Anonymous,
    Known(f64, f64) // latitude, longitude
}

impl Location {
    fn display(&self ) {
        match *self {
            Location::Unknown =>  println!("Unknown location"),
            Location::Anonymous =>  println!("Anonymous location"),
            Location::Known(lat,long ) =>  println!("{}, {}", lat, long)
        }
    }
}

fn main() {
    let location  = Location::Unknown;
    location.display();
    let location  = Location::Anonymous;
    location.display();
    let location  = Location::Known(28.608295, -80.604177);
    location.display();
}
