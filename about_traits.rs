
#[test]
fn can_be_defined_localy_but_only_once() {

    trait Incrementable {
        fn inc(&mut self);
    }

    let mut n : int = 1;

    (proc(n: &mut int) {
        impl Incrementable for int {
            fn inc(&mut self) { *self += 1; }
        }
        n.inc();
    })(&mut n);

    // Redefinition is a compile error
    //(proc(n: &mut int) {
    //    impl Incrementable for int {
    //        fn inc(&mut self) { *self += 1; }
    //    }
    //})(&mut n);

    assert!(n == 2);

    // compilation error
    //n.inc();

}


#[test]
fn can_have_static_functions() {
    
    trait Defaultable { fn default() -> Self; }
    impl Defaultable for int { fn default() -> int { 1 } }
    
    let one : int = Defaultable::default();

    assert!(one == 1);
}

#[test]
fn constructor_pattern_with_static_functions() {
    
    trait Shape { fn new(s: f64) -> Self; }
    struct Circle { radius: f64 }
    struct Square { length: f64 }
    
    impl Shape for Circle {
        fn new(radius: f64) -> Circle { Circle{ radius: radius } }
    }

    impl Shape for Square {
        fn new(length: f64) -> Square { Square{ length: length } }
    }

    let s:Square = Shape::new(5.0);
    let c:Circle = Shape::new(15.0);

    assert!(s.length == 5.0);
    assert!(c.radius == 15.0);

}
