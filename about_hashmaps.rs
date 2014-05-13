use collections::HashMap;

#[test]
fn hashmap_can_map_to_traits() {

    trait Getter {
        fn get(&self) -> int;
        fn set(&mut self, int);
    }
    struct GetElem { elem: int };
    impl Getter for GetElem {
        fn get(&self) -> int { return self.elem; }
        fn set(&mut self, val: int) { self.elem = val; }
    }

    let mut map : HashMap<int, ~Getter> = HashMap::new();

    let v : ~Getter = ~GetElem{ elem: 1 };
    map.insert(1, v);
    
    {
        let w = map.get_mut(&1);
        w.set(2);
    }

    let u = map.get(&1);
    assert_eq!(u.get(), 2);
}


