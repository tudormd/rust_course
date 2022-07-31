
use challenge_write_automated_test::*;
use rand::prelude::*;
use challenge_write_automated_test::Feature;

    #[test]
    fn it_rectangle_get_area(){
        let width = thread_rng().gen_range(0.1..99.9);
        let height = thread_rng().gen_range(0.1..99.9);
        let rect = Rectangle::new(width, height);
        let area = rect.get_feature(Feature::Area);
        assert_eq!(area, width * height)
    }

    #[test]
    fn it_rectangle_calc_perimeter(){
        let width = thread_rng().gen_range(0.1..99.9);
        let height = thread_rng().gen_range(0.1..99.9);
        let rect = Rectangle::new(width, height);
        let perimeter = rect.get_feature(Feature::Perimeter);
        assert_eq!(perimeter, 2 as f64 * width + 2.0 * height)
    }


    #[test]
    fn it_circle_get_area(){
        let radius = thread_rng().gen_range(0.1..99.9);
        let circle = Circle::new(radius);
        let area = circle.get_feature(Feature::Area);
        assert_eq!(area, std::f64::consts::PI * radius.powi(2))
    }

    #[test]
    fn it_circle_calc_circumference(){
        let radius = thread_rng().gen_range(0.1..99.9);
        let circle = Circle::new(radius);
        let perimeter = circle.get_feature(Feature::Perimeter);
        assert_eq!(perimeter, 2.0 * std::f64::consts::PI * radius)
    }
