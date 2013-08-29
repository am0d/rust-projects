use std::io;
use std::os;

fn main() {
    let args = os::args();
    io::println(~"hello world from '" + args[0] + "'!");
}
