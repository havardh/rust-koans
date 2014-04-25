#[test]
fn has_a_literal_notation() {
		let _v = [1, 2, 3];
}

#[test]
fn has_a_length_method() {
		let v = [1, 2];
		assert_eq!(v.len(), 2);
}

#[test]
fn can_be_initialized_with_a_capacity() {
		let vec: Vec<int> = Vec::with_capacity(5);
		assert_eq!(vec.capacity(), 5);
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
    let _v1 = [1,2,3];
    //let v2 = [1,2,4];

    //assert_eq!(v1, v1);
}



