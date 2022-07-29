struct Shuttle<'a> {
    name: &'a str
}
impl<'a, 'b> Shuttle<'a> {
    fn send_transmission(&'a self, msg: &'b str) -> &'b str {
        println!("Transmitting message: {}", msg);
        // self.name
        msg
    }
}


fn main() {
    let vehicle = Shuttle {
        name: "Endeavour"
    };

    let sender = vehicle.send_transmission("Greeting from orbit!");
    println!("sender is {sender}!");
}
