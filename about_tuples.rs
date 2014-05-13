mod tuples {

    #[test]
    fn can_be_created_easily() {

        let tuple = (1, 2);
        assert!((1, 2) == tuple);

    }

    #[test]
    fn items_are_extracted_by_deconstruction() {

        let (a, b) = (1, 2);

        assert!(a == 1);
        assert!(b == 2);

    }

    #[test]
    fn can_contain_multple_types() {

        let (a, b) = (1, 1.0);

        assert!(a == 1);
        assert!(b == 1.0);

    }

    #[test]
    fn can_have_names() {

        struct Tuple(int);
        let tuple = Tuple(1);

        let Tuple(a) = tuple;

        assert!(a == 1)
    }
}
