/*#[test]
fn function_parameter_can_only_be_called_once() {

    fn foo(f: |int|->int) {

        for i in range(0,10) {

            spawn(proc() {
                let _ = f(i);
            });
        }
    }

    let ident = |x| x;

    foo(ident);

}*/
