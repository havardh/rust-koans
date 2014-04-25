
struct Point {
		x: f64, 
		y: f64
}

#[test]
fn structs_can_be_constructed_with_names_parameters() {

		let p1 = Point{y:1.0, x:2.0};

		assert!(p1.x == 2.0);
		assert!(p1.y == 1.0);

}

#[test]
fn structs_can_be_constructed_without_names() {
		
		let p1 = Point{x:1.0, y:2.0};

		assert!(p1.x == 1.0);
		assert!(p1.y == 2.0);		

}

#[test]
fn structs_are_by_default_immutable() {
		
		let _p1 = Point{x:1.0, y:3.0};

		// _p1.x = 2.0; // this is a compilation error
		
		
}

#[test]
fn mutable_structs_are_created_by_the_mut_keyword() {
		
		let mut p1 = Point{x:0.0, y:0.0};
		p1.x = 1.0;

		assert!(p1.x == 1.0);

}
