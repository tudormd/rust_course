#[derive(Debug)]
struct Rectangle<T, U> { // Monomorphization
    width: T,
    height: U,
}

fn main() {
    let mut rect = Rectangle{
        width: 1u8,
        height: 3u16,
    };

    println!("Hello, world!, {:?}", rect);
}
