#[allow(non_implicitly_copyable_typarams)];

use send_map::linear;
use send_map::linear::LinearMap;

pub struct HttpHeaderCollection {
    mut header_collection: LinearMap<~str, ~[~str]>,
    mut status_code: int
}

pub fn HttpHeaderCollection() -> HttpHeaderCollection {
    return HttpHeaderCollection {
        header_collection: LinearMap(),
        status_code: 0
    }
}

impl HttpHeaderCollection {
    fn parse(&const self, headers: &str) {
        let lines = str::split_str_nonempty(str::replace(headers, ~"\r", ~""), ~"\n");

        let first_line = copy vec::head(lines);
        if str::starts_with(first_line, ~"HTTP/") {
            // parse the first line of the response
            // this should look something like:
            // HTTP/x.x XXX ...
            // where x.x is the HTTP version,
            // XXX is the status code, and ... is the status description
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

        // now parse the rest of the headers
        // format is:
        // Header-Name: Header-Contents
        //

        for vec::tail(lines).each_val |line| {
            if str::starts_with(line, " ") || str::starts_with(line, "\t") {
                // Header lines beginning with whitespace are a continuation
                // of the previous line. These are still a TODO item, as they are
                // apparently not so common in the real world anyway.
                debug!("Line is continuation of previous line: %s", line);
            }
            else {
                let mut parts = str::splitn_char(line, ':', 1);
                //debug!("Parts of header: %?", parts);
                parts[1] = str::trim(parts[1]);

                if self.header_collection.contains_key(&parts[0]) {
                    let mut values = self.header_collection.get(&parts[0]);
                    values = vec::append_one(values, copy parts[1]);
                    self.header_collection.insert(copy parts[0], move values);
                }
                else {
                    self.header_collection.insert(copy parts[0], ~[copy parts[1]]);
                }
            }
        }
        return
    }

    fn get_status_code(&const self) -> int {
        return self.status_code
    }

    fn get_header(&const self, header_name: ~str) -> ~str {
        // Get the value of a header, or empty string if not found
        // TODO Find a way to return multiple header values when they exist
        // (e.g. Set-Cookie which may appear multiple times).
        // Perhaps use an enum for the values?
        match self.header_collection.find (&header_name) {
            Some(value) => {
                copy value[0]
            },
            None => {
                ~""
            }
        }
    }

    fn to_str(&const self) -> ~str {
        // Converts the headers to a string representation
        // Note that this is not necessarily in the same order as they
        // were received in
        let mut header_string = ~"";
        let mut headers = copy self.header_collection;
        do headers.each |key, value| {
            header_string = str::append(header_string, 
            do vec::foldl(~"", *value) |prev_value, header_value| {
                str::append(prev_value, fmt!("%s: %s\n", *key, *header_value))
            });
            true
        }
        move header_string
    }
}
