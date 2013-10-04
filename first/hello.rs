use std::os;

fn main() {
    let args = os::args();
    println!("hello world from '{}'!", args[0]);
}
