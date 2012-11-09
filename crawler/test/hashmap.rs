use vec;
use send_map::linear;
use send_map::linear::LinearMap;

struct HeaderCollection {
    mut header_collection: LinearMap<~str, ~[~str]>
}

fn HeaderCollection () -> HeaderCollection {
    HeaderCollection {
        header_collection: LinearMap()
    }
}

impl HeaderCollection {
    fn parse(&mut self, line: ~str) {
        let mut parts = str::splitn_char(line, ':', 1);
        parts[1] = str::trim(parts[1]);
        debug!("Parts of header: %?", parts);
        if self.header_collection.contains_key(&parts[0]) {
            //self.header_collection.get(&parts[0]).append_one(copy parts[1]);
            debug!("Found key, appending now");
            let mut values = self.header_collection.get(&parts[0]);
            debug!("values before: %?", values);
            values = vec::append_one(values, copy parts[1]);
            debug!("values after: %?", values);
            self.header_collection.insert(copy parts[0], move values);
        }
        else {
            debug!("Key not found, inserting now");
            self.header_collection.insert(copy parts[0], ~[copy parts[1]]);
        }
        debug!("%?", self.header_collection);
    }
}

fn main() {
    let mut headers = HeaderCollection();
    headers.parse(~"New-Key: Value");
    headers.parse(~"New-Key: Value2");
}
