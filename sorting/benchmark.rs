#[link(name = "benchmark", vers = "0.1")];
#[crate_type = "lib"];

extern mod extra;
extern mod timer;
use std::rand;
use std::vec;
use extra::getopts::*;
use timer::Timer;

struct Benchmark {
    mut num_trials: uint,
    mut trial_size: uint,
    mut quiet: u8,
    mut parse_args: bool,
    mut verify: bool
}

pub impl Benchmark {
    fn new () -> Benchmark {
        Benchmark { 
            num_trials: 1, 
            trial_size: 10, 
            quiet: 0, 
            parse_args: true,
            verify: false
        }
    }
    
    fn parse_opts(&mut self) {
        if self.parse_args {
            let args = os::args();
            let opts = ~[
                optflagmulti("q"),
                optflag("quiet"),
                optopt("trialsize"),
                optopt("numtrials"),
                optflag("verify")
                ];
            let matches = match getopts(vec::tail(args), opts) {
                result::Ok(m) => { m }
                result::Err(f) => { fail!(fail_str(f)) }
            };
            if opt_present(&matches, "q") || opt_present(&matches, "quiet") {
                self.quiet = opt_count(&matches, "q") as u8;
                if self.quiet < 1{
                    self.quiet = 1;
                }
            }
            if opt_present(&matches, "verify") {
                self.verify = true;
            }

            match opt_maybe_str(&matches, "trialsize") {
                Some(size) => {
                    match uint::from_str(size) {
                        Some(ts) => { self.trial_size = ts }
                        None => { fail!("Trial size must be an integer") }
                    }
                }
                None => {}
            }

            match opt_maybe_str(&matches, "numtrials") {
                Some(trials) => {
                    match uint::from_str(trials) {
                        Some(t) => { self.num_trials = t }
                        None => { fail!("Number of trials must be an integer") }
                    }
                }
                None => {}
            }

            self.parse_args = false;
        }
    }

    fn run(&mut self, sort: ~fn(~[uint])->~[uint]) {
        self.parse_opts();
        let mut timer = Timer::new();
        let mut trial_number = 0;
        let mut sort_times = vec::from_elem(self.num_trials, 0);

        for self.num_trials.times {
            let vals = generate_random_array(self.trial_size);
            /* Run the sort and record the timing */
            match self.quiet {
                0 => { std::io::println("Starting sort ..."); }
                1 => { std::io::println(fmt!("Trial %?", trial_number)); }
                _ => {}
            }

            timer.start();
            let sorted = sort(vals);
            timer.end();

            match self.quiet {
                0 => { io::println("Sort finished."); }
                _ => {}
            }

            if self.verify {
                /* Check that it actually is sorted */
                match self.quiet {
                    0 => { std::io::println("Verifying sort ..."); }
                    _ => {}
                }
                if !ensure_sorted(sorted) {
                    /* Print the values so we can see what they actually look like.
                       Note: Should probably only do this if the array is small */
                    for sorted.each |v| {
                        io::println(fmt!("%?", *v as uint));
                    }
                    fail!(fmt!("Trial %?: Array was not sorted correctly", trial_number));
                }
                match self.quiet {
                    0 => { std::io::println("Sort was correct."); }
                    _ => {}
                }
            }

            /* Show the time it took */
            match self.quiet {
                0 => { timer.show_time(); }
                _ => {}
            }
            /* Record the time it took */
            sort_times[trial_number] = timer.get_total_time();

            trial_number += 1;
        }

        /* Print out the average time at the end */
        let total_time = do iter::sum |f| { sort_times.each(f) };
        let average_time = total_time / (self.num_trials as u64);
        io::println(fmt!("Average time: %s", timer::format_as_time(average_time)));
    }
}

pub fn generate_random_array(size: uint) -> ~[uint] {
    let ret = vec::build_sized(size, 
                    |push| {
                        for size.times {
                            push(rand::random());
                        }
                    }
                    );

    return ret;
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

