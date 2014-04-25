#[cfg(test)]
mod vec {

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

}
