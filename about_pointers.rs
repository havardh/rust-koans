#[test]
fn can_be_created_and_deferenced() {
    
    let a : ~int = ~0;
    assert_eq!(*a, 0);
    
}

#[test]
fn can_reside_a_lot_of_places() {
    
    let _on_the_stack =  1;
    let _wat          = &2;
    let _owned_box    = ~3;

}

mod owned {

    #[test]
    fn copying_is_deep() {
        
        let p1 :~int = ~0;
        { 
            let mut p2 = p1;

            assert_eq!(*p2, 0);
            *p2 = 1;
            assert_eq!(*p2, 1);
        }
        // p1 has been moved and this is a compilation error
        // assert_eq!(*p1, 0); 
    }

    #[test]
    fn name_of_test_case() {
        
    }
    
}

mod borrowed {

    #[test]
    fn copy_is_shallow() {
        
        let p1 : &mut int = &mut 0;
        {
            let p2 : &mut int = p1;

            assert_eq!(*p2, 0);
            *p2 = 1;
            assert_eq!(*p2, 1);
        }
        assert_eq!(*p1, 1);


    }

}
