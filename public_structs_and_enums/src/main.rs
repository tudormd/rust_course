fn main() {
    let rect = Shape::Rectangle::new(1.2, 3.4);
    println!("react area is {}", rect.get_area());
    println!("react width is {}", rect.width);
}

mod Shape {
    pub struct Rectangle {
    // pub (crate) struct Rectangle {
       pub width: f64,
        height: f64
    }

    impl Rectangle {
       pub fn new(width: f64, height: f64) -> Rectangle {
            Rectangle {
                width,
                height
            }
        }

        pub fn get_area(&self) -> f64 {
            self.width * self.height
        }
    }

}

