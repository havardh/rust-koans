
#[test]
fn supports_aritmetic_operators() {

    assert_eq!(1 + 1, 2);
    assert_eq!(1 - 1, 0);
    assert_eq!(2 * 2, 4);
    assert_eq!(4 / 2, 2);
    assert_eq!(4 % 2, 0);
    assert_eq!(-4, -4);

}

#[test]
fn logic_operators() {

    assert!(!false);
    assert!(true && true);
    assert!(false || true);

}

#[test]
fn operators_are_lazy() {

    assert!(true || (|| fail!("Newer Called"))());
    assert!(!(false && (|| fail!("Newer Called"))()));

    let mut x = 0;
    assert!(true && (|| { x += 1; true})());
    assert!(false || (|| { x += 1; true})());
    assert_eq!(x, 2);

}

#[test]
fn bitwise_operators() {

    assert_eq!(0b01 & 0b11, 0b01);
    assert_eq!(0b01 | 0b10, 0b11);
    assert_eq!(0b11 ^ 0b01, 0b10);
    assert_eq!(0b01 << 0b1, 0b10);
    assert_eq!(0b10 >> 0b1, 0b01);

}

#[test]
fn comparison_operators() {

    assert!(1 < 2);
    assert!(2 > 1);
    assert!(1 != 2);
    assert!(1 == 1);
    assert!(1 <= 1);
    assert!(2 >= 1);

}

#[test]
fn operators_are_implemented_as_traits() {

    #[deriving(Eq, Ord, Show)]
    struct Point{ x: f64, y: f64 }

    let p1 = Point{x: 1.0, y: 2.0};
    let p2 = Point{x: 2.0, y: 2.0};

    assert!(p1 == p1);
    assert!(p1 != p2);
    assert!(p1 < p2);


}
