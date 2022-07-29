use std::collections::HashMap;

fn main() {
    let mut missions_flow = HashMap::new(); // missions flown as of 1 Jan 2021
    missions_flow.insert("Hadfield", 3);
    missions_flow.insert("Hurley", 3);
    missions_flow.insert("Barron", 0);
    missions_flow.insert("Barron", 1);

    let kayla = missions_flow.entry("Barron").or_insert(0);

    *kayla +=1;

    missions_flow.entry("Stone").or_insert(2);

    println!("Hello, world!, {:?}",missions_flow);

    let barron_missions = missions_flow.get("Barron");
    println!("barron_missions!, {:?}",barron_missions);

}
