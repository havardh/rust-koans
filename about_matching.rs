#[test]
fn match_is_an_expression() {

    let x = match 0 {
        0 => 1,
        _ => 0
    };

    assert!(x == 1);
}

#[test]
fn match_can_use_or_and_range() {

    fn foo(x: int) -> int {
        match x {
            0 => 0,
            1 | 2 => 1,
            3..5 => 2,
            _ => 3
        }
    }

    assert!(foo(0) == 0);
    assert!(foo(1) == 1);
    assert!(foo(4) == 2);

}

#[test]
fn match_can_bind_to_locals() {

    let x = match 100 {
        x if x < 50 => x+1,
        x if x > 50 => x-1,
        _ => 0
    };

    assert!(x == 99);
}

#[test]
fn binding_can_also_be_done_for_ranges() {
    let x = match 3 {
        a @ 0..4 => a,
        _ => 5
    };

    assert!(x == 3);
}


#[test]
fn can_include_destructuring_of_enums() {

    enum T { A(int, int), B(int, int) }

    let b = B(1, 2);

    let (n, m) = match b {
        A(n, m) => (n, m),
        B(n, m) => (m, n)
    };

    assert_eq!(n, 2);
    assert_eq!(m, 1);

}

#[test]
fn can_match_() {

    fn reverse(original: ~str) -> ~str {

        match original.as_slice() {
            //[_, ..rest] => ~"",
            _ => "".to_owned(),
        }

    }


    reverse("test".to_owned());
}
