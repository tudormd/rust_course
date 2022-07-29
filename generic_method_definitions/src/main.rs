#[derive(Debug)]
struct Rectangle<T, U> { // Monomorphization
    width: T,
    height: U,
}

impl<T, U> Rectangle<T, U> {
    fn get_width(&self) -> &T {
        &self.width
    }
}

impl Rectangle<u8, u8> {
    fn get_perimeter(&self) -> u8 {
        2 * self.width  + 2 * self.height
    }
}

fn main() {
    let mut rect = Rectangle{
        width: 1u8,
        height: 1u8,
    };

    println!("Hello, world!, {:?}", rect);
    println!("Hello, world!, {:?}", rect.get_width());
    println!("Hello, world!, {:?}", rect.get_perimeter());
}
