#[allow(non_implicitly_copyable_typarams)];

use core::io::{Writer,WriterUtil,Reader,ReaderUtil};
use url=std::net_url;
use std::net::url::*;
use ip=std::net_ip;
use std::net_ip::{Ipv4,Ipv6,IpAddr};
use std::uv_global_loop;
use socket=std::net_tcp;

struct Request {
    mut url: Url,
    mut headers: headers::HttpHeaderCollection,
    mut response_text: Option<~str>
}

pub fn Request(requestUrl: ~str) -> Request {
    Request {
        url: url::from_str(requestUrl).get(),
        headers: headers::HttpHeaderCollection(),
        response_text: None
    }
}

impl Request {
    fn get () -> ~str {
        // perform a GET request for the resource.
        // TODO: move most of this out to a separate function so that it
        // can be shared with POST, PUT, HEAD, DELETE, etc.
        let ip_address = {
            let ip_address = get_ip_address(self.url);
            if ip_address.is_ok() {
                result::unwrap(ip_address)
            } else {
                return ~""
            }
        };

        let connection = {
            let connection = socket::connect(ip_address, 80, uv_global_loop::get());
            if connection.is_ok() {
                socket::socket_buf(result::unwrap(connection))
            } else {
                return ~""
            }
        };

        let writer = connection as Writer;
        writer.write_str(build_request(self.url));
        writer.flush();

        let reader = connection as Reader;
        let mut headers = ~"";
        let mut end_of_header = false;
        while !end_of_header {
            let line = reader.read_line();
            //debug!("%s", line);
            if str::trim_chars(line, ['\r', ' ']).is_empty() {
                end_of_header = true;
            }
            headers = str::concat([headers, ~"\n", line]);
        }
        debug!("%s", headers);
        self.headers.parse(headers);

        match self.headers.get_status_code() {
            302 => {
                io::println(fmt!("Redirecting to %s", self.headers.get_header(~"Location")));
                let new_url = url::from_str(self.headers.get_header(~"Location")).get();
                self.url = new_url;
                self.headers = headers::HttpHeaderCollection();
                debug!("Getting now ...");
                return self.get()
            },
            _ => {
                debug!("Getting stream");
                let mut response = ~"";
                let mut end_of_response = false;
                while !reader.eof() {
                    let line = reader.read_line();
                    debug!("%s", line);
                    response = str::concat([response, ~"\n", line]);
                }
                debug!("Got response");
                self.response_text = Some(response);
            }
        }
        ~""
    }

    fn get_content_type() -> ~str {
        return self.headers.get_header(~"Content-Type")
    }

    fn get_status_code() -> int {
        return self.headers.get_status_code()
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

    return request_header;
}

fn get_ip_address (url: Url) -> Result<IpAddr, ~str> {
    let resolution = ip::get_addr(url.host, uv_global_loop::get());

    if resolution.is_ok() {
        debug!("Host resolution successful");
        let ip_addrs = result::unwrap(resolution);
        if ip_addrs.is_not_empty() {
            let best_ip = do (ip_addrs).find |ip| {
                match ip {
                    Ipv4(*) => { true }
                    Ipv6(*) => { false }
                }
            };

            if best_ip.is_some() {
                return Ok(option::unwrap(best_ip));
            } else {
                return Err(~"No suitable ip address to resolve to");
            }
        } else {
            return Err(~"No ip address found for host");
        }
    } else {
        debug!("Host resolution error: %?", resolution);
        return Err(~"Host unknown");
    }
}
