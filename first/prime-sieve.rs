extern mod std;
use core::task::spawn;
use core::comm::{stream,Chan,Port};

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

    //let mut i = 0;
    loop {
        let prime = prev_port.recv();
        io::println(fmt!("%d", prime));

        let (new_port, new_chan) = stream();
        do spawn |prev_port| {
            filter(&prev_port, &new_chan, prime);
        }
        prev_port = new_port;
        //i += 1;
    }
}
