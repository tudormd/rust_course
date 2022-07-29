#[derive(Debug)]
enum Shape {
    Circle(f64), // radius
    Rectangle(f64, f64), // with, height
    Triangle(f64, f64, f64), // sides, a, b, c
}

fn main() {
    let my_shape = Shape::Rectangle(1.2 , 3.4);
    println!("Hello, world!, {:?}", my_shape);

    match my_shape {
        Shape::Circle(r) => print!("Circle with radius {}", r),
        Shape::Rectangle(w, h) => print!("{} x {} Rectangle", w, h),
        Shape::Triangle(a, b, c) => print!("Triangle sides {}, {}, {}", a, b, c),
    }
}

