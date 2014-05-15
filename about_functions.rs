#[test]
fn a_function_speficies_parameters_and_return_types() {

    fn ident(x: int) -> int {
        return x;
    }

    assert_eq!(ident(1), 1);

}

#[test]
fn the_return_keyword_can_be_omitted() {

    fn ident(x: int) -> int { x }

    assert_eq!(ident(2), 2);

}


#[test]
fn function_is_pass_by_copy_for_pods() {

    let a = 1;

    fn test(mut x: int) -> int {
        x = 2;
        return x;
    }

    let x = test(a);

    assert_eq!(a, 1);
    assert_eq!(x, 2);
}

#[test]
fn void_functions_have_no_return_type() {

    let a = 1;

    fn add_one(mut x: int) {
        x += 1;

        assert_eq!(x, 2);
    }

    add_one(a);

    assert_eq!(a, 1);

}

#[test]
fn references_must_be_dereferenced() {

    fn foo(x: &int) {
        assert_eq!(*x, 2);
    }

    foo(&2);
}

#[test]
fn pass_by_refernce_is_done_by_ampersand() {

    let mut x = 1;

    fn add_one(x: &mut int) {
        *x += 1;
    }

    add_one(&mut x);

    assert_eq!(x, 2);

}

#[test]
fn a_managed_pointer_is_moved_on_call() {

    let x = box 1;

    fn foo(x: Box<int>) {
        assert_eq!(*x, 1);
    }

    foo(x);

    //assert_eq!(*x, 1); // compilation error as x is moved

}

#[test]
fn functions_can_be_generic() {

    fn ident<T>(x: T) -> T { x }

    assert_eq!(ident::<f64>(1.0), 1.0);

}

//#[test]
//fn generic_functions_can_specify_trait_constraints() {
//
//    fn neg<T: Neg<T>>(x: T) -> T { -x }
//
//}

#[test]
fn some_traits_take_parameters() {

    fn add<T: Add<T,T>>(a: T, b: T) -> T { a + b }

    assert_eq!(add::<int>(1,2), 3);
    assert_eq!(add::<f64>(1.0,2.0), 3.0);

}
