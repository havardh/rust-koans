#[test]
fn can_be_defined_on_existing_types_through_traits() {
    
    trait Incrementable {
        fn inc(&mut self);
    }

    impl Incrementable for int {
        fn inc(&mut self) { *self += 1; }
    }

    let mut n : int = 1;
    n.inc();

    assert!(n == 2);

}

