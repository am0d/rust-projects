extern mod std;
use std::net_ip;
use std::uv_global_loop;

fn main() {
    let url = "www.google.com";
    let iotask = uv_global_loop::get();
    let address = net_ip::get_addr(url, iotask);
    match address {
        Ok(ips) => {
            for ips.each |ip| {
                io::println(net_ip::format_addr(ip));
            };
        },
        Err(_) => io::println(fmt!("Error: " ))
    };
    return;
}
