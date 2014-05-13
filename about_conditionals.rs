
#[test]
fn must_contain_braces() {
    let mut a;

    if 1 == 1 {
        a = true;
    } else {
        a = false;
    };

    assert!(a == true);
}

#[test]
fn else_part_may_be_omitted() {

    let mut a = 1;

    if 1 != 1 {
        a = 2;
    }

    assert_eq!(a, 1);
}

#[test]
fn else_if_is_supported() {

    let mut a;

    if false {
        a = 2;
    } else if true {
        a = 3;
    }   else {
        a = 4;
    };

    assert_eq!(a, 3);

}

#[test]
fn if_test_can_be_used_as_expressions() {

    assert!(if 2 > 3 { false } else { true } == true)

}

#[test]
fn semicolons_at_end_of_block_returns_unit() {

    assert!( if true { true; } else { false; } == () )

}

#[test]
fn condition_must_be_a_bool() {

    let mut a;
    if 1 != 0 {
        a = true;
    } else {
        a = false;
    };

    assert!(a == true);

}
