use std::io::stdin;
use std::io::ReaderUtil;

fn main() {
    println("What is your name?");
    let input = stdin();
    let name = input.read_line();
    println!("Hello {}", name);
}
