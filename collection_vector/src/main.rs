fn main() {
    let mut astronauts: Vec<String> = Vec::new();
    astronauts.push(String::from("Shepard"));
    astronauts.push(String::from("Grissom"));
    astronauts.push(String::from("Glenn"));

    println!("Hello, world!, {:?}", astronauts);

    let last =     astronauts.pop();
    println!("Hello, last!, {:?}", last);

    // let third = &astronauts[2];
    let third = &astronauts.get(2);
    println!("Hello, third!, {:?}", third);

    let countdown = vec![5, 4,3,2,1];
}
