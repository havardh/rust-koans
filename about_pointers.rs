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
