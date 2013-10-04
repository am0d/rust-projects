use std::task::spawn;
use std::comm::{stream, SharedChan};

fn main () {
    let (port, chan): (Port<int>, Chan<int>) = stream();
    let chan: SharedChan<int> = SharedChan::new(chan);

    for child_number in range(0, 20) {
        let child_chan: SharedChan<int> = chan.clone();
        do spawn {
            child_chan.send(child_number);
        }
    }

    for _ in range(0, 20) {
        let received = port.recv();
        println!("Message received from child {}", received);
    }
}
