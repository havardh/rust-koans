
#[test]
fn literal_is_not_vector() {
    // let v : Vec<int> = [1, 2, 3]; compilation
}

#[test]
fn has_convenient_initializer_macro() {
    let vec1 = vec!(2, 1);
    assert_eq!(*vec1.get(0), 2);
    assert_eq!(*vec1.get(1), 1);
}

#[test]
fn has_a_length_method() {
    let v = vec!(1, 2);
    assert_eq!(v.len(), 2);
}

#[test]
fn can_be_initialized_with_a_capacity() {
    let vec: Vec<int> = Vec::with_capacity(5);
    assert_eq!(vec.capacity(), 5);
}

#[test]
fn has_a_constructor_with_infered_type() {

    let mut v = Vec::new();

    v.push(1);
    v.push(2);

    assert_eq!(*v.get(0), 1);
    assert_eq!(*v.get(1), 2);
}
#[test]
fn has_get_method_to_get_immutable_reference() {

    let v : Vec<int> = Vec::from_slice([1,2]);
    assert_eq!(*v.get(0), 1);
    assert_eq!(*v.get(1), 2);
}


#[test]
fn can_be_appended_to() {
    let vec1 = vec!(1, 2, 3);
    let vec1 = vec1.append([4, 5, 6]);
    assert_eq!(vec1.len(), 6);
}

#[test]
fn mutable_allows_for_push() {
    let mut vec = vec![1,2];
    vec.push(3);
    vec.push(4);

    assert_eq!(vec.len(), 4);
}

#[test]
fn can_be_compared() {
    let v1 = vec!(1,2,3);
    let v2 = vec!(1,2,4);

    assert!(v1 == v1);
    assert!(v1 != v2);
}

#[test]
fn can_be_constructed_from_functions() {

    let actual = Vec::from_fn(3, |a| a*10);
    assert_eq!(actual, vec!(0, 10, 20));

}

#[test]
fn to_contain() {

    let v1 : Vec<int> = vec!(1,2,3);
    assert!(v1.iter().filter(|x:&&int| **x == 1).len() > 0)

}
