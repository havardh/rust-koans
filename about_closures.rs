
mod stack {
    #[test]
    fn have_a_compact_syntax() {

        assert_eq!((|| 1)(), 1);

    }

    #[test]
    fn have_untyped_parameters() {
        assert_eq!((|a| a)(42), 42);
    }

    #[test]
    fn closes_as_expected() {

        let var = "hi";
        let ident = | x | x;

        assert_eq!(ident(var), var);

    }

    #[test]
    fn can_be_returned_from_function() {

        fn func() -> (|| -> &'static str) {
            || ""
        }

        assert_eq!(func()(), "");

    }

    #[test]
    fn cannot_be_returned_from_stack_frame() {

        // compilation error
        //fn func(x: int) -> (|| -> int) {
        //    || -> int x
        //}

        //assert_eq!(func(1)(), 1);

    }

    #[test]
    fn used_as_parameters_in_functions() {

        let a = vec!(1, 2, 3);

        let b = a.iter().map(|&x| x).collect();

        assert!(a == b);
        assert!(b == a);

    }

}

mod owned {

    #[test]
    fn can_be_returned_from_stack_frame() {

        fn func(x: int) -> (proc() -> int) {
            proc() -> int { return x }
        }

        assert_eq!(func(1)(), 1);

    }

}
