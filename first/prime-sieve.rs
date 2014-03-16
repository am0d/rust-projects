use std::task::spawn;
use std::comm::channel;
use std::cell::RefCell;

fn generate(sender: &Sender<int>) {
    let mut i = 2;
    loop {
        sender.send(i);
        i = i + 1;
    }
}

fn filter(receiver: &Receiver<int>, sender: &Sender<int>, prime: int) {
    loop {
        let i = receiver.recv();
        if i % prime != 0 {
            sender.send(i);
        }
    }
}

fn main() {
    let (sender, receiver) = channel();

    let mut prev_receiver = receiver;

    spawn(proc() {
        generate(&sender);
    });

    loop {
        let prime = prev_receiver.recv();
        println!("{}", prime);

        let (new_sender, new_receiver) = channel();
        let prev_receiver_cell = RefCell::new(prev_receiver);

        spawn(proc() {
            filter(&prev_receiver_cell.unwrap(), &new_sender, prime);
        });
        prev_receiver = new_receiver;
    }
}
