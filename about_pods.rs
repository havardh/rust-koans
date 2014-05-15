#[test]
fn a_struct_defaults_to_beeing_pod() {

    struct Point{ x: int, y: int };

    let p = Point{x: 1, y: 2};
    let _q = p; // copy

    assert_eq!(p.x, 1);

}

#[test]
fn a_struct_with_a_owning_pointer_is_not_a_pod() {

    struct Point{ x: Box<int>, y: Box<int> };

    let p = Point{x: box 1, y: box 2};
    let _q = p; // move

    //assert_eq!(*p.x, 1); // ERROR. as p is moved

}
