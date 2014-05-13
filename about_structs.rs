
struct Point {
    x: f64,
    y: f64
}

#[test]
fn structs_can_be_constructed_with_names_parameters() {

    let p1 = Point{y:1.0, x:2.0};

    assert!(p1.x == 2.0);
    assert!(p1.y == 1.0);

}

#[test]
fn structs_can_be_constructed_without_names() {

    let p1 = Point{x:1.0, y:2.0};

    assert!(p1.x == 1.0);
    assert!(p1.y == 2.0);

}

#[test]
fn structs_are_by_default_immutable() {

    let _p1 = Point{x:1.0, y:3.0};

    // _p1.x = 2.0; // this is a compilation error


}

#[test]
fn mutable_structs_are_created_by_the_mut_keyword() {

    let mut p1 = Point{x:0.0, y:0.0};
    p1.x = 1.0;

    assert!(p1.x == 1.0);

}

#[test]
fn mutablility_is_inherited() {

    struct A { b: B };
    struct B { val: int };
    let mut a = A { b: B { val: 1 } };

    a.b.val = 2;

    assert_eq!(a.b.val, 2);
}

#[test]
fn can_define_private_members() {

    struct Object {
        priv_x: int
    }

    let obj = Object{ priv_x: 1 };

    impl Object {
        fn get_x(&self) -> int { self.priv_x }
    }

    // assert_eq!(obj.priv_int, 1); compilation error
    assert_eq!(obj.get_x(), 1);

}
