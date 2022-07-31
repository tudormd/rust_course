use challenge_write_automated_test::*;

fn main() {
    let rect = Rectangle::new(1.0, 2.0);
    let area = rect.get_feature(Feature::Area);
    println!("rect area is {}", area);

    let circ = Circle::new(3.0);
    let perimeter = circ.get_feature(Feature::Perimeter);
    println!("circ perimeter is {}", perimeter);

}
