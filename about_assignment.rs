
#[test]
fn variables_are_single_assignment() {
    let _a = 1;
    //_a = 2; // compilation error

}

#[test]
fn mutable_variables_are_created_with_mut_keyword() {
    let mut a = 1;
    assert!(a == 1);

    a = 2;
    assert!(a == 2);
}
