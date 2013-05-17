#[link(name = "benchmark", vers = "0.1")]
#[crate_type = "lib"]

extern mod std;
use core::rand;
use core::vec;

pub fn generate_random_array(size: uint) -> ~[uint] {
    let ret = vec::build_sized(size, |push| {
        for size.times {
            push(rand::random());
        }
    });

    return ret;
}

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
    pub fn get_time_string(&mut self) -> ~str {
        let MIN_MULTIPLIER:u64 = 60 * 1000 * 1000 * 1000;
        let SEC_MULTIPLIER:u64 = 1000 * 1000 * 1000;

        let total_time = self.end_time - self.start_time;
        let minutes = total_time / MIN_MULTIPLIER;
        let seconds = (total_time - minutes * MIN_MULTIPLIER) / SEC_MULTIPLIER;
        let nanoseconds = (total_time - minutes * MIN_MULTIPLIER - seconds * SEC_MULTIPLIER);

        let mut time_string = ~"";
        if minutes > 0 {
            time_string += fmt!("%?:", minutes);
        }
        if minutes > 0 || seconds > 0 {
            if seconds < 10 {
                // HACK: fmt!("%02?.", seconds) doesn't zero pad
                time_string += "0";
            }
            time_string += fmt!("%?.", seconds);
        }
        time_string += fmt!("%.5?", nanoseconds);

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
    pub fn show_time(&mut self) -> () {
        core::io::println(fmt!("Total time: %s", self.get_time_string()));
    }
}

fn ensure_sorted(arr: &[uint]) -> bool {
    let mut previous_value = 0;
    for arr.each |v| {
        if *v < previous_value {
            return false
        }
        previous_value = *v;
    }
    true
}

pub fn run(num: uint, sort: ~fn(&mut [uint])) -> () {
    let mut timer = Timer::new();
    let mut vals = generate_random_array(num);

    /* Run the sort and record the timing */
    core::io::println("Starting sort ...");

    timer.start();
    sort(vals);
    timer.end();

    core::io::println("Sort finished, verifying ...");

    /* Check that it actually is sorted */
    if !ensure_sorted(vals) {
        /* Print the values so we can see what they actually look like.
           Note: Should probably only do this if the array is small */
        for vals.each |v| {
            core::io::println(fmt!("%?", *v as uint));
        }
        fail!("Array was not sorted correctly");
    }

    /* Show the time it took */
    core::io::println("Sort was correct.");
    timer.show_time();
}
