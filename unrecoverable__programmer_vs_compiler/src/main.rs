fn main() {
    for x in -10 ..10 {
        dbg!(x, even_or_add(x));
    }
}


fn even_or_add(x: i32)-> String {
    match x % 2 {
        0 => "even".to_string(),
        1 | -1 => "odd".to_string(),
        _ => unreachable!() // impossible, will panic if reachable
    }
}

