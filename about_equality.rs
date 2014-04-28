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

#[test]
fn equal_operator_is_not_reflecsive() {

		let a = [1, 2, 3];
		let b = ~[1, 2, 3];
		

		assert!(a == a);
		assert!(b == b);
		assert!(a == b);
		// assert!(b == a); compilation error
		// assert!(b == ~a); compilation error
		
}
