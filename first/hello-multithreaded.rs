use std::task::spawn;
use std::comm::channel;

fn main () {
    let (sender, receiver) = channel::<int>();

    for child_number in range(0, 20) {
        let child_sender = sender.clone();
        spawn(proc() {
            child_sender.send(child_number);
        });
    }

    for _ in range(0, 20) {
        let received = receiver.recv();
        println!("Message received from child {}", received);
    }
}
