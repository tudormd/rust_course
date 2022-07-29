#[derive(Debug)]
enum Shape {
    Circle(f64), // radius
    Rectangle(f64, f64), // with, height
    Triangle(f64, f64, f64), // sides, a, b, c
}

// impl Shape {
//     fn get_parameters(&self) -> f64 {
//         match *self {
//             Shape::Circle(r) => r * 2.0 * std::f64::consts::PI,
//             Shape::Rectangle(w, h) =>  (2.0 * w) + (2.0 * h),
//             Shape::Triangle(a, b, c) => a+ b+ c
//         }
//     }
// }

trait Description {
    fn get_parameters(&self) -> f64;
}

impl Description  for Shape {
    fn get_parameters(&self) -> f64 {
        match *self {
            Shape::Circle(r) => r * 2.0 * std::f64::consts::PI,
            Shape::Rectangle(w, h) =>  (2.0 * w) + (2.0 * h),
            Shape::Triangle(a, b, c) => a+ b+ c
        }
    }
}

fn main() {
    let my_shape = Shape::Rectangle(1.2 , 3.4);
    let perimeter = my_shape.get_parameters();
    println!("Hello, world!, {:?}", perimeter);
}
