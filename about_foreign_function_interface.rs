use libc;

#[test]
fn the_c_std_lib_is_included() {

    fn ascii_val(ch: char) -> i32 { ch.to_ascii().to_byte() as i32 }

    let a = ascii_val('A');
    let space = ascii_val(' ');

    assert_eq!(unsafe { libc::isalpha( a ) }, 1);
    assert_eq!(unsafe { libc::isalpha( space ) }, 0);

}

#[test]
fn functions_can_be_included_from_so_files() {

    #[link(name="id")]

    extern { fn id(x: libc::c_int) -> libc::c_int; }

    assert_eq!(unsafe{ id(10) }, 10);

}
