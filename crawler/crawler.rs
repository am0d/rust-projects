use request::Request;

fn main() {
    let r = Request(~"http://www.google.com/");
    let _ = r.get();

    io::println(fmt!("Status: %d", r.get_status_code()));
    io::println(r.headers.to_str());
    io::println(r.response_text.get());
    return;
}
