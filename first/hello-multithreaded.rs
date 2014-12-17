use std::task::spawn;
use std::comm::channel;

fn main () {
    let (sender, receiver) = channel::<int>();

    for child_number in range(0, 20i) {
        let child_sender = sender.clone();
        spawn(move || {
            child_sender.send(child_number);
        });
    }

    for _ in range(0, 20i) {
        let received = receiver.recv();
        println!("Message received from child {}", received);
    }
}
