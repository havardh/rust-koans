
#[test]
fn variables_are_assigned_in_let_statements() {

    let x = 0;

    assert_eq!(x, 0);

}

#[test]
fn variables_are_single_assignment() {
    let a = 1;
    //_a = 2; // compilation error

    assert_eq!(a, 1);
}

#[test]
fn mutable_variables_are_created_with_mut_keyword() {
    let mut a = 1;
    assert!(a == 1);

    a = 2;
    assert!(a == 2);
}

#[test]
fn values_can_be_copied() {

    let a = 1;
    let b = a;

    assert_eq!(b, 1);

}

#[test]
fn can_be_redefined() {

    let _a = 1;
    let _a = 2;

    assert_eq!(_a, 2);

}

#[test]
fn can_be_assigned_value_from_expression() {

    let a = 1 + 2;

    assert_eq!(a, 3);

}

#[test]
fn type_is_specifed_in_let_statement() {

    use core::any::Any;
    use std::intrinsics::TypeId;

    let a : int = 100;

    assert_eq!(TypeId::of::<int>(), a.get_type_id());

}
