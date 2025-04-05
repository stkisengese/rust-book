use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64, pub f64);

impl Point {
    pub fn distance(&self, other: Point) -> f64 {
        let dx = self.0 - other.0;
        let dy = self.1 - other.1;
        (dx.powi(2) + dy.powi(2)).sqrt()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}


impl Circle {
    pub fn new(x: f64, y: f64, radius: f64) -> Self {
        Circle {
            center: Point(x, y),
            radius,
        }
    }
    
    pub fn diameter(&self) -> f64 {
        self.radius * 2.0
    }

    pub fn area(&self) -> f64 {
        PI * self.radius.powi(2)
    }
    pub fn intersect(&self, other: Circle) -> bool {
        self.center.distance(other.center) <= (self.radius + other.radius)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let circle = Circle::new(500.0, 500.0, 150.0);
        let circle1 = Circle {
            center: Point(80.0, 115.0),
            radius: 30.0,
        };
        let point_a = Point(1.0, 1.0);
        let point_b = Point(0.0, 0.0);  
        assert_eq!(circle.area(), 70685.83470577035);
        assert_eq!(circle.diameter(), 300.0);
        assert_eq!(circle1.diameter(), 60.0);
        assert_eq!(circle.intersect(circle1), false);
        assert_eq!(point_a.distance(point_b), 1.4142135623730951);
    }
}
