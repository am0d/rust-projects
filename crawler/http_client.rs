use to_str::to_str;
use ptr::to_unsafe_ptr;
use comm::{Port, Chan};
use result::{Result, Ok, Err};
use std::net::ip::{
    get_addr, format_addr,
    IpAddr, IpGetAddrErr, Ipv4, Ipv6
};
use std::net::tcp::{connect, TcpSocket};
use std::net::url;
use std::net::url::Url;
use std::uv_global_loop;
use connection::{
    Connection, ConnectionFactory, UvConnectionFactory,
    MockConnection, MockConnectionFactory
};
use parser::{Parser, ParserCallbacks};
use request::build_request;

pub const timeout: uint = 2000;

/// HTTP status codes
pub enum StatusCode {
    StatusOk = 200,
    StatusFound = 302,
    StatusUnknown
}

/// HTTP request error conditions
pub enum RequestError {
    ErrorDnsResolution,
    ErrorConnect,
    ErrorMisc
}

/// Request 
pub enum RequestEvent {
    Status(StatusCode),
    Payload(~mut Option<~[u8]>),
    Error(RequestError)
}

impl StatusCode: cmp::Eq {
    pure fn eq(other: &StatusCode) -> bool {
        self as uint == (*other) as uint
    }
    pure fn ne(other: &StatusCode) -> bool {
        self as uint != (*other) as uint
    }
}

impl RequestError: cmp::Eq {
    pure fn eq(other: &RequestError) -> bool {
        self as uint == (*other) as uint
    }
    pure fn ne(other: &RequestError) -> bool {
        self as uint != (*other) as uint
    }
}

impl RequestEvent: cmp::Eq {
    pure fn eq(other: &RequestEvent) -> bool {
        // FIXME: bad copy
        match (copy self, copy *other) {
          (Status(a), Status(b)) => a == b,
          (Payload(a), Payload(b)) => a == b,
          (Error(a), Error(b)) => a == b,

          (Status(*), _)
          | (Payload(*), _)
          | (Error(*), _) => false
        }
    }
    pure fn ne(other: &RequestEvent) -> bool {
        !self.eq(other)
    }
}

pub type DnsResolver = fn@(host: ~str) -> Result<~[IpAddr], IpGetAddrErr>;

pub fn uv_dns_resolver() -> DnsResolver {
    |host: ~str| {
        let iotask = uv_global_loop::get();
        get_addr(host.to_str(), iotask)
    }
}

pub fn uv_http_request(url: Url) -> HttpRequest<TcpSocket, UvConnectionFactory> {
    HttpRequest(uv_dns_resolver(), UvConnectionFactory, move url)
}

#[allow(non_implicitly_copyable_typarams)]
pub struct HttpRequest<C: Connection, CF: ConnectionFactory<C>> {
    resolve_ip_addr: DnsResolver,
    connection_factory: CF,
    url: Url,
    parser: Parser,
    mut cb: fn@(+ev: RequestEvent)
}

pub fn HttpRequest<C: Connection, CF: ConnectionFactory<C>>(resolver: DnsResolver,
                                                            connection_factory: CF,
                                                            url: Url) ->
                                                            HttpRequest<C,CF> {
    HttpRequest {
        resolve_ip_addr: move resolver,
        connection_factory: move connection_factory,
        url: move url,
        parser: move Parser(),
        cb: |_event| { }
    }
}

#[allow(non_implicitly_copyable_typarams)]
impl<C: Connection, CF: ConnectionFactory<C>> HttpRequest<C, CF> {
    fn begin() -> Result<~str, ~str> {
        debug!("http_client: looking up url %?", self.url.to_str());
        let ip_addr = match self.get_ip() {
          Ok(addr) => { copy addr }
          Err(e) => { 
             // cb(Error(e)); 
              return Err(fmt!("http_client: unable to resolve address: %?", e));
          }
        };

        debug!("http_client: using IP %? for %?", format_addr(&ip_addr), self.url.to_str());

        let socket = {
            #debug("http_client: connecting to %?", ip_addr);
            let socket = self.connection_factory.connect(copy ip_addr, 80);
            if socket.is_ok() {
                result::unwrap(move socket)
            } else {
                #debug("http_client: unable to connect to %?: %?", ip_addr, socket);
                //cb(Error(ErrorConnect));
                return Err(fmt!("http_client: unable to connect to %?: %?", ip_addr, socket));
            }
        };

        debug!("http_client: got socket for %?", ip_addr);

        let request_header = build_request(copy self.url);
        debug!("http_client: writing request header: %?", request_header);
        let request_header_bytes = str::to_bytes(request_header);
        match socket.write_(move request_header_bytes) {
          result::Ok(*) => { }
          result::Err(e) => {
            // FIXME: Need test
            //cb(Error(ErrorMisc));
            return Err(fmt!("http_client: unable to write to socket: %?", e));
          }
        }

        let read_port = {
            let read_port = socket.read_start_();
            if read_port.is_ok() {
                result::unwrap(move read_port)
            } else {
                //cb(Error(ErrorMisc));
                return Err(fmt!("http_client: unable get port for reading: %?", read_port));
            }
        };

        let mut response = ~"";
        loop {
            let next_data = read_port.recv();

            if next_data.is_ok() {
                let next_data = result::unwrap(move next_data);
                //debug!("data: %?", str::from_bytes(next_data));
                response = str::append(move response, str::from_bytes(next_data));
            } else {
                #debug("http_client: read error: %?", next_data);

                // This method of detecting EOF is lame
                match next_data {
                  result::Err({err_name: ~"EOF", _}) => {
                    debug!("Reached end of file");
                    break;
                  }
                  _ => {
                    // FIXME: Need tests and error handling
                    socket.read_stop_(read_port);
                    //cb(Error(ErrorMisc));
                    return Err(fmt!("http_client: read error: %?", next_data));
                  }
                }
            }
        }
        socket.read_stop_(read_port);
        return Ok(move response);
    }

    fn get_ip() -> Result<IpAddr, RequestError> {
        let ip_addrs = self.resolve_ip_addr(copy self.url.host);
        if ip_addrs.is_ok() {
            let ip_addrs = result::unwrap(move ip_addrs);
            // FIXME: This log crashes
            //#debug("http_client: got IP addresses for %?: %?", self.url, ip_addrs);
            if ip_addrs.is_not_empty() {
                // FIXME: Which address should we really pick?
                let best_ip = do ip_addrs.find |ip| {
                    match ip {
                      Ipv4(*) => { true }
                      Ipv6(*) => { false }
                    }
                };

                if best_ip.is_some() {
                    return Ok(option::unwrap(move best_ip));
                } else {
                    // FIXME: Need test
                    return Err(ErrorMisc);
                }
            } else {
                #debug("http_client: got no IP addresses for %?", self.url);
                // FIXME: Need test
                return Err(ErrorMisc);
            }
        } else {
            #debug("http_client: DNS lookup failure: %?", ip_addrs.get_err());
            return Err(ErrorDnsResolution);
        }
    }
}
