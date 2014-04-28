
use collections::HashMap;

#[test]
fn has_map_features() {
    
    let mut map : HashMap<int,int> = HashMap::new();

    map.insert(5, 7);
    assert_eq!(map.len(), 1);
    assert_eq!(*map.get(&5), 7);
    assert_eq!(*map.get(&5), 7);

}

#[test]
fn has_matchable_result() {
    
    let map : HashMap<int,int> = HashMap::new();

    match map.find(&0) {
        Some(_) => fail!(),
        None => assert!(true)
    }

}

#[test]
fn can_be_mutated_inplace() {
    
    let mut map : HashMap<int,int> = HashMap::new();
    map.insert(0, 1);

    (|| { let x = map.get_mut(&0); *x = 42; })();
    
    assert_eq!(*map.get_mut(&0), 42);

}
