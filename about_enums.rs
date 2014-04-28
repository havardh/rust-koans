use std::f64;

#[test]
fn different_size_can_be_passed_to_functions() {
    
    struct Point {x: f64, y: f64 };
    struct Size {w: f64, h: f64 };
    enum Shape {
        Circle(Point, f64),
        Rectangle(Point, Size)
    }

    fn area(shape: Shape) -> f64 {
        match shape {
            Circle(_, r) => r*r * f64::consts::PI,
            Rectangle(_, Size{w,h}) => w * h
        }
    }

    assert_eq!(area(Circle(Point{x:0.0,y:0.0}, 1.0)), f64::consts::PI);
    assert_eq!(area(Rectangle(Point{x:0.0, y:0.0}, Size{w:2.0,h:4.0})), 8.0);

}
