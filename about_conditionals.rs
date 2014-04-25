mod conditionals {

		#[test]
		fn if_test_can_be_used_as_expressions() {
				
				assert!(if 2 > 3 { 0 } else { 1 } == 1)

		}

		#[test]
		fn semicolons_at_end_of_block_returns_unit() {
				
				assert!( if true { true; } else { false; } == () )

		}

		#[test]
		fn condition_must_be_a_bool() {
				
				if 1 != 0 {
						assert!(true);
				} else {
						assert!(false);
				}

		}

}
