fn main() {
    let message = String::from("Greetings from Earth!");
    println!("Hello, world!, {}", message);

    let last_worlde = &message[15..15+5];
    let last_world = &message[15..];

    println!("Hello, world!, {}", last_world);

    let planets = [1, 2, 3, 4, 5, 6, 7, 8];
    let inner_planets: &[i32] = &planets[..4];
    println!("Hello, world!, {:?}", inner_planets);

}
