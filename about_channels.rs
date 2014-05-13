extern crate sync;


#[test]
fn are_created_with_channel_function() {
    let (_tx, _rx) : (Sender<int>, Receiver<int>) = channel();
}

#[test]
fn have_send_and_recive_primitives() {

    let (tx, rx) : (Sender<int>, Receiver<int>) = channel();

    tx.send(1);
    let val = rx.recv();

    assert_eq!(val, 1);

}

#[test]
fn used_to_communicate_between_tasks() {

    let (tx, rx) : (Sender<int>, Receiver<int>) = channel();

    spawn(proc() { tx.send(100); });

    assert_eq!(rx.recv(), 100);
}

#[test]
fn an_endpoint_cannot_be_shared() {

    let (tx, rx) : (Sender<int>, Receiver<int>) = channel();

    spawn(proc() { tx.send(100); });
    assert_eq!(rx.recv(), 100);

    // Compilation error.
    //spawn(proc() {
    //    tx.send(101);
    //    assert_eq!(rx.recv(), 101);
    //});


}

#[test]
fn a_clone_enables_sharing_send_endpoint() {

    let (tx, rx) : (Sender<int>, Receiver<int>) = channel();

    for val in range(0, 4) {

        let tx_clone = tx.clone();
        spawn(proc() { tx_clone.send(val); });

    }

    let val = rx.recv() + rx.recv() + rx.recv() + rx.recv();

    assert_eq!(val, 0+1+2+3);

}

#[test]
fn bidirectional_onces_exists() {

    fn child(channel: &sync::DuplexStream<int, int>) {
        for _ in range(1,10) {
            channel.send(channel.recv() + 1);
        }
    }

    let (from, to) = sync::duplex();

    spawn(proc() { child(&to) });


    for i in range(1,10) {
        from.send(i);
        assert_eq!(from.recv(), i+1);
    }

}


#[test]
fn channels_are_async() {

    let (tx, _rx) = channel();
    tx.send(1);
    assert!(true);

}
