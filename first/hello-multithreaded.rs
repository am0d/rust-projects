use std::io;
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

    let mut count = 0;

    loop {
        if count >= 20 {
            break;
        }

        let received = port.recv();
        io::print(fmt!("Message received from child %d\n", received));
        count = count + 1;
    }
}
