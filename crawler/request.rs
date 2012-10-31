use io::{Writer,WriterUtil,Reader,ReaderUtil};
use std::net::url;
use std::net::ip;
use socket=std::net::tcp;

enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE
}

struct Request {
    url: Url,
     method: HttpMethod
}

pub fn Request(requestUrl: ~str) -> Request {
    Request {
        url: url::from_str(requestUrl).get(),
        method: GET
    }
}

impl Request {
    fn get () -> ~str {
        let ip_address = {
            let ip_address = get_ip_address(self.url);
            if ip_address.is_ok() {
                result::unwrap(move ip_address)
            } else {
                return ~""
            }
        };

        let connection = {
            let connection = socket::connect(ip_address, 80, uv_global_loop::get());
            if connection.is_ok() {
                socket::socket_buf(result::unwrap(move connection))
            } else {
                return ~""
            }
        };

        let writer = connection as Writer;
        debug!("Writing");
        writer.write_str(build_request(self.url));
        debug!("Written");
        writer.flush();
        debug!("Flushed");

        let reader = connection as Reader;
        loop {
            let response = reader.read_line();  //crashes here due to bug #3891
            debug!("%s", response);
        }

        return ~"";
   //     return str::from_bytes(response);
    }
}

fn build_request(url: Url) -> ~str {
    let host = copy url.host;
    let mut path = 
        if url.path.is_not_empty() { 
            copy url.path 
        } else { 
            ~"/" 
        };

    if url.query.len() > 0 {
        let kvps = do url.query.map |pair| {
            match *pair {
                (key, value) => fmt!("%s=%s", key, value)
            }
        };
        path += ~"?" + str::connect(kvps, "&");
    }

    let request_header = fmt!("GET %s HTTP/1.0\u000D\u000AHost: %s\u000D\u000AUser-Agent: rust::requests\u000D\u000A\u000D\u000A",
            path, host);

    return move request_header;
}

fn get_ip_address (url: Url) -> Result<IpAddr, ~str> {
    let resolution = ip::get_addr(url.host, uv_global_loop::get());

    if resolution.is_ok() {
        debug!("Host resolution successful");
        let ip_addrs = result::unwrap(move resolution);
        if ip_addrs.is_not_empty() {
            let best_ip = do ip_addrs.find |ip| {
                match ip {
                    Ipv4(*) => { true }
                    Ipv6(*) => { false }
                }
            };

            if best_ip.is_some() {
                return Ok(option::unwrap(move best_ip));
            } else {
                return Err(~"No suitable ip address to resolve to");
            }
        } else {
            return Err(~"No ip address found for host");
        }
    } else {
        debug!("Host resolution error: %?", resolution);
        match resolution.get_err() {
            GetAddrUnknownErr => {
                return Err(~"Host unknown");
            }
        }
    }
}
