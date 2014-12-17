extern crate time;

const SEC_MULTIPLIER:u64 = 1000 * 1000 * 1000;
const MIN_MULTIPLIER:u64 = 60 * SEC_MULTIPLIER;
const HR_MULTIPLIER:u64 = 60 * MIN_MULTIPLIER;

#[deriving(Copy)]
pub struct Timer {
    start_time: u64,
    end_time: u64
}

impl Timer {
    pub fn new() -> Timer {
        Timer { start_time: 0, end_time: 0}
    }
    pub fn start(&mut self) -> () {
        self.start_time = time::precise_time_ns();
    }
    pub fn end(&mut self) -> () {
        self.end_time = time::precise_time_ns();
    }
    pub fn get_time_string(&mut self) -> String {
        return format_as_time(self.get_total_time());
    }
    pub fn get_total_time(&mut self) -> u64 {
        return self.end_time - self.start_time;
    }
    pub fn show_time(&mut self) -> () {
        println!("Total time: {:}", self.get_time_string());
    }
}

pub fn format_as_time(total_time: u64) -> String {
    let hours = total_time / HR_MULTIPLIER;
    let minutes = (total_time 
                   - hours * HR_MULTIPLIER) 
        / MIN_MULTIPLIER;
    let seconds = (total_time 
                   - hours * HR_MULTIPLIER 
                   - minutes * MIN_MULTIPLIER) 
        / SEC_MULTIPLIER;
    let nanoseconds = total_time 
                       - hours * HR_MULTIPLIER 
                       - minutes * MIN_MULTIPLIER 
                       - seconds * SEC_MULTIPLIER;

    let mut time_string = String::new();
    if hours > 0 {
        time_string.push_str(format!("{}:", hours as int).as_slice());
    }
    if hours > 0 || minutes > 0 {
        if minutes < 10 && hours > 0 {
            time_string.push_str("0");
        }
        time_string.push_str(format!("{}:", minutes as int).as_slice());
    }
    if hours > 0 || minutes > 0 || seconds > 0 {
        if seconds < 10 && (minutes > 0 || hours > 0) {
            // HACK: format!("%02?.", seconds) doesn't zero pad
            time_string.push_str("0");
        }
        time_string.push_str(format!("{}.", seconds as int).as_slice());
        // nanoseconds don't need to be quite as accurate if we measure seconds
        let ns_as_string = format!("{:.5}", (nanoseconds as f64) / (SEC_MULTIPLIER as f64));
        time_string.push_str(format!("{:}", ns_as_string.as_slice().slice(2, 5)).as_slice());
    } else {
        time_string.push_str(format!("{:}", format_number(nanoseconds)).as_slice());
    }

    if hours > 0 {
        time_string.push_str(" hr");
    } else if minutes > 0 {
        time_string.push_str(" min");
    } else if seconds > 0 {
        time_string.push_str(" sec");
    } else {
        time_string.push_str(" ns");
    }

    //time_string += format!(" ({})", total_time);

    return time_string;
}

fn format_number(num: u64) -> String {
    let repr = num.to_string();
    let repr = repr.as_slice();
    let mut ret_val = String::new();
    let mut index = 0;
    let length = repr.len();

    while index < length {
        ret_val.push_str(repr.slice(index, index + 1));

        if (length - index - 1) % 3 == 0 && length > index + 1{
            ret_val.push_str(",");
        }
        index += 1;
    }

    return ret_val
}

#[test]
fn format_number_test() {
    let num1 = 123456789;
    let num2 = 12345678;
    let num3 = 1234;

    assert!(format_number(num1) == "123,456,789");
    assert!(format_number(num2) == "12,345,678");
    assert!(format_number(num3) == "1,234");
}

#[test]
fn format_as_time_test() {
    let num1 = 2000;    // ns
    let num2 = 3 * SEC_MULTIPLIER + 141591234;
    let num3 = 1 * MIN_MULTIPLIER + 5 * SEC_MULTIPLIER + 98765432;
    let num4 = 3 * HR_MULTIPLIER + num3;

    assert!(format_as_time(num1).as_slice() == "2,000 ns");
    assert!(format_as_time(num2).as_slice() == "3.141 sec");
    assert!(format_as_time(num3).as_slice() == "1:05.098 min");
    assert!(format_as_time(num4).as_slice() == "3:01:05.098 hr");
}
