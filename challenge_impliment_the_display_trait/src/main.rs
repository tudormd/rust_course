use std::fmt; // Import `fmt`
use std::cmp::{PartialEq, PartialOrd, Ordering};
use std::cmp::Ordering::{Greater, Equal, Less};

 #[derive(PartialEq)]
struct Satellite {
    name: String,
    velocity: f64 // miles per second
}

// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
impl fmt::Display for Satellite {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}, {}", self.name, self.velocity)
    }
}

impl PartialOrd for Satellite {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if  self.velocity < other.velocity {
           return Some(Less)
        } else if  self.velocity > other.velocity {
            return Some(Greater)
        }

        Some(Equal)
    }
}


fn main() {
    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 7.8
    };

    let gps = Satellite {
        name: String::from("GPS"),
        velocity: 7.6
    };

    println!("Display: {:?}", hubble.partial_cmp(&gps));
}
