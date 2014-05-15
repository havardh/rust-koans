#[test]
fn can_be_created_and_deferenced() {

    let a : Box<int> = box 0;
    assert_eq!(*a, 0);

}

#[test]
fn can_reside_a_lot_of_places() {

    let _on_the_stack =  1;
    let _wat          = &2;
    let _owned_box    = box 3;

}

mod a_owned_pointer {

    #[test]
    fn is_moved_on_assigment() {

        let p1 : Box<int> = box 0;
        {
            let mut p2 = p1;

            assert_eq!(*p2, 0);
            *p2 = 1;
            assert_eq!(*p2, 1);
        }
        // p1 has been moved and this is a compilation error
        //assert_eq!(*p1, 0);
    }

    #[test]
    fn coerses_to_a_reference() {

        let p1 = box 1;
        let p2 : &int = p1;

        assert_eq!(*p1, *p2);

    }

}

mod a_reference {

    #[test]
    fn is_copied_on_assignment() {

        let p1 : &mut int = &mut 0;
        {
            let p2 : &mut int = p1;

            assert_eq!(*p2, 0);
            *p2 = 1;
            assert_eq!(*p2, 1);
        }
        assert_eq!(*p1, 1);
    }

    #[test]
    fn coerses_to_a_owned_pointer() {

        let p1 = box 1;
        let p2 : &int = p1;

        assert_eq!(*p1, *p2);

    }

}
