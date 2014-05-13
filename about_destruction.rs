//#[test]
//fn testy() {
//
//    struct Type {
//        a : Option<int>
//    }
//
//    let mut y : int;
//    match Type{ a: Some(1) } {
//        Type { Some(x) } => y = x,
//        Type { None } => 0
//    }
//
//    assert!(y == 1);
//
//}

#[test]
fn tuples_inner_values() {

    let tuple = (Some(1), Some(2));

    let mut x;
    let mut y;

    match tuple {
        (Some(a), Some(b)) => { x = a; y = b; },
        (Some(a), None) => { x = a; y = -1; },
        (None, Some(b)) => { x = -1; y = b; },
        (None, None) => { x = -1; y = -1; },
    }

    assert!(x == 1);
    assert!(y == 2);

}
