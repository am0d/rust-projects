extern mod std;
use io::ReaderUtil;

fn main() {
    io::println(~"What is your name?");
    let r = io::stdin();
    let name = r.read_line();
    io::println(fmt!("Hello %s", name));
}
