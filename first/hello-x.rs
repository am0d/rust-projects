use std::rt::io::stdin;
use std::rt::io::buffered::BufferedReader;

fn main() {
    println("What is your name?");
    let mut input = BufferedReader::new(stdin());
    let name = input.read_line();
    println!("Hello {}", name);
}
