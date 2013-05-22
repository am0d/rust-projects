#[link(name = "timer", vers = "0.1")]
#[crate_type = "lib"]

extern mod std;

pub struct Timer {
    mut start_time: u64,
    mut end_time: u64
}

pub impl Timer {
    pub fn new() -> Timer {
        Timer { start_time: 0, end_time: 0}
    }
    pub fn start(&mut self) -> () {
        self.start_time = std::time::precise_time_ns();
    }
    pub fn end(&mut self) -> () {
        self.end_time = std::time::precise_time_ns();
    }
    fn get_time_string(&mut self) -> ~str {
        return format_as_time(self.get_total_time());
    }
    fn get_total_time(&mut self) -> u64 {
        return self.end_time - self.start_time;
    }
    pub fn show_time(&mut self) -> () {
        core::io::println(fmt!("Total time: %s", self.get_time_string()));
    }
}

pub fn format_as_time(total_time: u64) -> ~str {
    let MIN_MULTIPLIER:u64 = 60 * 1000 * 1000 * 1000;
    let SEC_MULTIPLIER:u64 = 1000 * 1000 * 1000;

    let minutes = total_time / MIN_MULTIPLIER;
    let seconds = (total_time - minutes * MIN_MULTIPLIER) / SEC_MULTIPLIER;
    let nanoseconds = (total_time - minutes * MIN_MULTIPLIER - seconds * SEC_MULTIPLIER);

    let mut time_string = ~"";
    if minutes > 0 {
        time_string += fmt!("%?:", minutes);
    }
    if minutes > 0 || seconds > 0 {
        if seconds < 10 && minutes > 0 {
            // HACK: fmt!("%02?.", seconds) doesn't zero pad
            time_string += "0";
        }
        time_string += fmt!("%?.", seconds);
        // nanoseconds don't need to be quite as accurate if we measure seconds
        time_string += fmt!("%.5?", nanoseconds);
    } else {
        time_string += fmt!("%s", format_number(nanoseconds));
    }

    if minutes > 0 {
        time_string += " min";
    } else if seconds > 0 {
        time_string += " sec";
    } else {
        time_string += " ns";
    }

    //time_string += fmt!(" (%?)", total_time);

    return time_string;
}

fn format_number(num: u64) -> ~str {
    let repr = num.to_str();
    let mut ret_val = ~"";
    let mut index = 0;
    let length = str::len(repr);

    while index < length {
        ret_val += str::slice(repr, index, index + 1);

        if (length - index - 1) % 3 == 0 && length > index + 1{
            ret_val += ",";
        }
        index += 1;
    }

    return ret_val
}
