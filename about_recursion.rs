#[test]
fn support_basic_recursion() {

    fn fac(n: int) -> int {
        match n {
            1 => 1,
            n => n * fac(n-1)
        }
    }

    assert_eq!(fac(5), 5*4*3*2*1);

}
