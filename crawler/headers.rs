use std::map;
use std::map::HashMap;

pub struct HttpHeaderCollection {
    mut header_collection: HashMap<~str, ~str>,
    mut status_code: int
}

pub fn HttpHeaderCollection() -> HttpHeaderCollection {
    return HttpHeaderCollection {
        header_collection: HashMap(),
        status_code: 0
    }
}

impl HttpHeaderCollection {
    fn parse(&mut self, headers: &str) {
        let lines = str::split_str_nonempty(headers, ~"\n");
        let first_line = copy vec::head(lines);
        if str::starts_with(first_line, ~"HTTP/") {
            debug!("Response header: %s", first_line);
            let parts = str::split_char(first_line, ' ');
            assert vec::len(parts) > 2;
            let status_code = int::from_str(parts[1]);
            match status_code {
                Some(s) => {
                    self.status_code = s
                },
                None => {
                    debug!("Status code should be an integer");
                    self.status_code = 0
                }
            }
        }
        else {
            debug!("Incorrect header line!");
            debug!("%?", first_line);
        }
        return
    }

    fn get_status_code(&mut self) -> int {
        return self.status_code
    }
}

impl HttpHeaderCollection: ToStr {
    pure fn to_str() -> ~str {
        return ~""
    }
}
