extern mod extra;
use std::task::spawn;
use std::comm::{Chan,Port};
use std::cell::RefCell;

fn generate(ch: &Chan<int>) {
    let mut i = 2;
    loop {
        ch.send(i);
        i = i + 1;
    }
}

fn filter(in_ch: &Port<int>, out_ch: &Chan<int>, prime: int) {
    loop {
        let i = in_ch.recv();
        if i % prime != 0 {
            out_ch.send(i);
        }
    }
}

fn main() {
    let (port, chan) = Chan::new();

    let mut prev_port = port;

    spawn(proc() {
        generate(&chan);
    });

    loop {
        let prime = prev_port.recv();
        println!("{}", prime);

        let (new_port, new_chan) = Chan::new();
        let prev_port_cell = RefCell::new(prev_port);

        spawn(proc() {
            filter(&prev_port_cell.unwrap(), &new_chan, prime);
        });
        prev_port = new_port;
    }
}
