extern mod std;
use core::task::spawn;
use core::comm::{stream, SharedChan};

fn main () {
    let (port, chan): (Port<int>, Chan<int>) = stream();
    let chan: SharedChan<int> = SharedChan::new(chan);

    for int::range(0, 20) |child_number| {
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
