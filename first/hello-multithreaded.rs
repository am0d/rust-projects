use std::task::spawn;
use std::comm::SharedChan;

fn main () {
    let (port, chan): (Port<int>, SharedChan<int>) = SharedChan::new();

    for child_number in range(0, 20) {
        let child_chan: SharedChan<int> = chan.clone();
        spawn(proc() {
            child_chan.send(child_number);
        });
    }

    for _ in range(0, 20) {
        let received = port.recv();
        println!("Message received from child {}", received);
    }
}
