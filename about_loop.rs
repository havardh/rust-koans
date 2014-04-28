mod fors {

    #[test]
    fn iterates_over_iterables() {
        
        let arr : Vec<~int> = vec!{~1,~3,~5};

        let mut sum = 0;
        for &~i in arr.iter() {
            sum += i;
        }

        assert_eq!(sum, 1+3+5);

    }

    #[test]
    fn range_is_includsive_exclusive() {

        let mut a = 0;

        for i in range(0, 4) {
            assert!(i < 4);
            assert!(i >= 0);
            a += 1;
        }

        assert_eq!(a, 4);
        
    }

}

mod loops {

    #[test]
    fn defines_the_while_true() {
        
        loop { break; }
        assert!(true);

    }

}
