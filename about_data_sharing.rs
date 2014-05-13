use sync::Arc;

#[test]
fn immutable_data_can_be_shared() {

    let ident = |x|x;
    let numbers = Vec::from_fn(100, ident);
    let numbers_arc = Arc::new(numbers.clone());

    let (rtx, rrx) = channel();

    for _i in range(1,10) {
        let (tx, rx) = channel();
        tx.send(numbers_arc.clone());

        let local_rtx = rtx.clone();
        spawn(proc() {

            let local_arc : Arc<Vec<uint>> = rx.recv();
            let numbers = &*local_arc;

            local_rtx.send(numbers.iter().fold(0, |a,b| a+*b));

        });
    }

    let expected = numbers.iter().fold(0, |a,b| a+*b);

    for _i in range(1,10) {
        assert_eq!(rrx.recv(), expected);
    }

}
