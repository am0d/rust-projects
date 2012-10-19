extern mod std;

fn main() {
    let args = os::args();
    io::println(~"hello world from '" + args[0] + "'!");
}
