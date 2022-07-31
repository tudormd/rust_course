fn is_even(number: i32) -> bool {
    match number % 2 {
        0 => true,
        _ => false
    }
}

fn add_two(a:i32, b:i32) -> i32 {
    a + b
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn it_fails() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_is_even(){
        assert!(is_even(42));
        assert!(!is_even(41));
    }

    #[test]
    fn test_with_assert(){
        let result = add_two(2, 2);
        assert!( result == 4, "Expected 4, result was {}", result);
    }

    #[test]
    fn test_with_assert_with_eq(){
        let result = add_two(2, 2, );
        assert_eq!(result, 4, "Expected 4, result was {}", result);
    }

    #[test]
    fn test_with_assert_with_ne(){
        assert_ne!(add_two(2, 2), 3);
    }

    #[test]
    #[should_panic(expected = "A person needs a name!")]
    fn person_text(){
        let new_person = Person::new("".to_string());
        assert_ne!(new_person.name, "Steve".to_string());
    }
}


pub struct Person {
    name: String
}

impl Person {
    pub fn new(name: String) -> Person{
        if name.is_empty(){
            panic!("A person needs a name!")
        }

        Person{
            name
        }
    }
}
