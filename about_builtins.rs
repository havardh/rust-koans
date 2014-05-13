
mod integers {

    #[test]
    fn has_a_max_value() {

        let n : u8 = 255;

        assert_eq!(n, ::std::u8::MAX);

    }

    #[test]
    fn has_a_min_value() {

        assert_eq!(0, ::std::u8::MIN);

    }

    #[test]
    fn wrap_on_overflow() {

        let mut n : int = ::std::int::MAX;
        n += 1;
        assert_eq!(n, ::std::int::MIN);

    }



}
