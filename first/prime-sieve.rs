extern mod extra;
use std::task::spawn_with;
use extra::comm::{SyncPort, SyncChan, rendezvous};
use std::iter::count;

fn generate(ch: SyncChan<int>) {
    for i in count(2, 1) {
        if !ch.try_send(i) {
            break;
        }
    }
}

fn filter(in_ch: SyncPort<int>, out_ch: SyncChan<int>, prime: int) {
    loop {
        let i = in_ch.recv();
        if i % prime != 0 {
            out_ch.send(i);
        }
    }
}

fn main() {
    let (port, chan) = rendezvous();

    let mut prev_port = port;

    do spawn_with(chan) |chan| {
        generate(chan);
    }

    loop {
        let prime = prev_port.recv();
        println!("{}", prime);

        let (new_port, new_chan) = rendezvous();

        do spawn_with((prev_port, new_chan)) |(prev_port, new_chan)| {
            filter(prev_port, new_chan, prime);
        }
        prev_port = new_port;
    }
}
