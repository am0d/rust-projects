use request::Request;

fn main() {
    let r = Request(~"http://example.com/index.php?test=0");
    let response = r.get();

    io::println(response);
    return;
}
