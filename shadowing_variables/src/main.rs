fn main() {
    let planet: &str = "Earth";
    {
        let planet = "Mars";
        println!("Hello, world! {}", planet);
    }
    println!("Hello, world! {}", planet);
}
