#[allow(non_implicitly_copyable_typarams)];

use request::Request;

fn main() {
    let r = Request(~"http://www.google.com/");
    let _ = r.get();

    io::println(fmt!("Status: %d", r.get_status_code()));
    io::println(fmt!("Content-Type: %s", r.get_content_type()));
    io::println(r.response_text.get());
    return;
}
