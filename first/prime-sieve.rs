extern mod extra;
use std::task::spawn;
use std::comm::{stream,Chan,Port};
use std::cell::Cell;

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
    let (port, chan) = stream();

    let mut prev_port = port;

    do spawn {
        generate(&chan);
    }

    loop {
        let prime = prev_port.recv();
        println!("{}", prime);

        let (new_port, new_chan) = stream();
        let prev_port_cell = Cell::new(prev_port);

        do spawn {
            filter(&prev_port_cell.take(), &new_chan, prime);
        }
        prev_port = new_port;
    }
}
