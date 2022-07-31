pub enum Feature {
    Area,
    Perimeter
}

#[derive(PartialEq, Debug)]
pub struct Rectangle {
    width: f64,
    height: f64
}

#[derive(PartialEq, Debug)]
pub struct Circle {
    radius: f64,
}

impl Circle {
    pub fn new(radius: f64) -> Circle {
        Circle {
            radius,
        }
    }

    pub fn get_feature(&self, feature: Feature) -> f64 {
        match feature {
            Feature::Area => self.calc_area(),
            Feature::Perimeter => self.calc_circumference(),
        }
    }

    fn calc_area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }

    fn calc_circumference(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}



impl Rectangle {
    pub fn new(width: f64, height: f64) -> Rectangle {
        Rectangle {
            width,
            height
        }
    }

    pub fn get_feature(&self, feature: Feature) -> f64 {
        match feature {
            Feature::Area => self.calc_area(),
            Feature::Perimeter => self.calc_perimeter(),
        }
    }

    fn calc_area(&self) -> f64 {
        self.width * self.height
    }

    fn calc_perimeter(&self) -> f64 {
        2.0 * self.width + 2.0 * self.height
    }

}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_new_rectangle(){
        let rect = Rectangle::new(1.0, 2.0);
        assert_eq!(rect, Rectangle {width: 1.0,height: 2.0});
    }

    #[test]
    fn ut_rectangle_get_area(){
        let rect = Rectangle::new(1.0, 2.0);
        let area = rect.get_feature(Feature::Area);
        assert_eq!(area, 2.0)
    }

    #[test]
    fn ut_rectangle_calc_perimeter(){
        let rect = Rectangle::new(1.0, 2.0);
        let perimeter = rect.get_feature(Feature::Perimeter);
        assert_eq!(perimeter, 6.0)
    }


    #[test]
    fn ut_new_circle(){
        let circle = Circle::new(1.0);
        assert_eq!(circle, Circle {radius: 1.0 });
    }

    #[test]
    fn ut_circle_get_area(){
        let circle = Circle::new(1.0);
        let area = circle.get_feature(Feature::Area);
        assert_eq!(area, 3.141592653589793)
    }

    #[test]
    fn ut_circle_calc_circumference(){
        let circle = Circle::new(1.0);
        let perimeter = circle.get_feature(Feature::Perimeter);
        assert_eq!(perimeter, 6.283185307179586)
    }


}


