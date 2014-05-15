use std::str::from_byte;
use std::str::from_char;

#[test]
fn can_be_tested_for_equality() {

    assert_eq!("hei", "hei");

}

#[test]
fn also_when_the_are_actual_strings() {

    assert_eq!("hei".to_owned(), "hei".to_owned());

}

#[test]
fn characters_can_be_extracted_as_u8() {

    let string = "String".to_owned();

    assert_eq!(83u8, string[0]);

}



#[test]
fn can_be_iterated_over() {

    fn reverse(original: &str) -> ~str {

        let mut string = "".to_owned();

        for ch in original.chars() {
            string = from_char(ch) + string;
        }

        return string;

    }

    assert_eq!("reverse".to_owned(), reverse("esrever"));

}

#[test]
fn can_be_used_as_slices() {

    fn reverse(original: &str) -> ~str {

        match original.len() {
            0 => "".to_owned(),
            n => reverse(original.slice(1, n)) + from_byte(original[0])
        }
    }

    assert_eq!("reverse".to_owned(), reverse("esrever"));

}
