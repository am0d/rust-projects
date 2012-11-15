extern mod std;
use ip=std::net_ip;
use tcp=std::net_tcp;
use std::uv_global_loop;

fn main() {
    let ip_address = match ip::get_addr("example.com", uv_global_loop::get()) {
        Ok(m) => copy m,
            _ => fail
    }.head();

    io::println(fmt!("%?", ip_address));

    let _ = {
        let connection = tcp::connect(move ip_address, 80, uv_global_loop::get());
        if connection.is_ok() {
            move result::unwrap(move connection)
        } else {
            return 
        }
    };

}
