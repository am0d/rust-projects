use request::Request;

fn main() {
    let r = Request(~"http://www.google.com/robots.txt");
    let response = r.get();

    io::println(fmt!("%?", response));
    return;
}
