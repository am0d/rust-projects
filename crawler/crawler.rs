extern mod std;
use std::{net_tcp, net_ip};
use std::uv_global_loop;

fn build_request() -> ~str {
    let request = ~"GET / HTTP 1.0\u000D\u000A";
    return move request;
}

fn main() {
    let url = "localhost";
    let iotask = uv_global_loop::get();
    let address = net_ip::get_addr(url, iotask);
    match address {
        Ok(ips) => {
            for ips.each |ip| {
                io::println(net_ip::format_addr(ip));
            };
            io::println("");
            for ips.each |ip| {
                io::println(net_ip::format_addr(ip));
                let ip_addr = copy ip;
                let socket = {
                    let socket = std::net::tcp::connect(*ip_addr, 80, iotask);
                    if socket.is_ok() {
                        result::unwrap(move socket)
                    }
                    else {
                        io::println("Error connecting");
                        return;
                    }
                };
                io::println("Connected!");
                match socket.write(str::to_bytes(build_request())) {
                    Ok(_) =>  {}
                    Err(e) => {
                        io::println(fmt!("Error sending request: %?", e));
                        return;
                    }
                };
                io::println("Wrote data");
                loop {
                    io::println("starting loop");
                    let data = socket.read(1000);
                    io::println("Received some data");
                    if data.is_ok() {
                        let data = result::unwrap(move data);
                        io::println(str::from_bytes(data));
                    }
                    else {
                        let err = result::unwrap_err(move data);
                        io::println(fmt!("Read error: %?", err));
                        break;
                    }
                };
            };
        },
        Err(_) => io::println(fmt!("Error: " ))
    };
    return;
}
