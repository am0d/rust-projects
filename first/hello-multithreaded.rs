extern mod std;
use task::spawn;
use pipes::{stream, SharedChan};

fn main () {
    let (chan, port) = stream();
    let chan = SharedChan(move chan);

    for int::range(0, 20) |child_number| {
        let child_chan = chan.clone();
        do spawn |move child_chan| {
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
