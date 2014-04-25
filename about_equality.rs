mod equality {

		#[test]
		fn the_equals_operator_requires_the_eq_trait() {
				
				struct Type{t: int}

				impl Eq for Type {
						fn eq(&self, other: &Type) -> bool {
								self.t == other.t
						}
				}

				let (a, b) = (Type{t:1}, Type{t:2});

				assert!(a == a);
				assert!(a != b);

		}

}
