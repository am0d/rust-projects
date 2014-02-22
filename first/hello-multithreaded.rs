use std::task::spawn;
use std::comm::Chan;

fn main () {
    let (port, chan): (Port<int>, Chan<int>) = Chan::new();

    for child_number in range(0, 20) {
        let child_chan: Chan<int> = chan.clone();
        spawn(proc() {
            child_chan.send(child_number);
        });
    }

    for _ in range(0, 20) {
        let received = port.recv();
        println!("Message received from child {}", received);
    }
}
