use std::os;

/// The main entry point for the application
fn main() {
    let args = os::args();
    println!("hello world from '{}'!", args.get(0));
}
