extern crate getopts;
extern crate time;

use std::{result, os};
use std::rand::random;
use std::iter::AdditiveIterator;
use getopts::{getopts, optflag, optflagmulti, optopt, usage};
use timer::Timer;

pub mod timer;

pub struct Benchmark {
    num_trials: uint,
    trial_size: uint,
    quiet: u8,
    parse_args: bool,
    verify: bool
}

impl Benchmark {
    pub fn new () -> Benchmark {
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
            let opts = &[
                optflagmulti("q", "quiet", "Provide less output"),
                optopt("", "trialsize", "Number elements to sort in each trial", ""),
                optopt("", "numtrials", "Number of trials to perform", ""),
                optflag("", "verify", "Verify that the sort was correct"),
                optflag("h", "help", "Show this help")
                ];
            let matches = match getopts(args.tail(), opts) {
                result::Ok(m) => { m }
                result::Err(f) => { panic!(f.to_string()) }
            };
            if matches.opt_present("h") || matches.opt_present("help") {
                let brief = format!("Usage: {} [options]", args.as_slice().head().map(|x| x.as_slice()).unwrap_or(""));
                print!("{}", usage(brief.as_slice(), opts));
                self.num_trials = 0;
                self.parse_args = false;
                return;
            }
            if matches.opt_present("q") {
                self.quiet = matches.opt_count("q") as u8;
                if self.quiet < 1{
                    self.quiet = 1;
                }
            }
            if matches.opt_present("verify") {
                self.verify = true;
            }

            match matches.opt_str("trialsize") {
                Some(size) => {
                    match from_str::<uint>(size.as_slice()) {
                        Some(ts) => { self.trial_size = ts }
                        None => { panic!("Trial size must be an integer") }
                    }
                }
                None => {}
            }

            match matches.opt_str("numtrials") {
                Some(trials) => {
                    match from_str::<uint>(trials.as_slice()) {
                        Some(t) => { self.num_trials = t }
                        None => { panic!("Number of trials must be an integer") }
                    }
                }
                None => {}
            }

            self.parse_args = false;
        }
    }

    pub fn run(&mut self, sort: fn(Vec<uint>) -> Vec<uint>) {
        self.parse_opts();
        let mut timer = Timer::new();
        let mut sort_times = Vec::with_capacity(self.num_trials);

        for trial_number in range(0, self.num_trials) {
            let vals = generate_random_array(self.trial_size);
            /* Run the sort and record the timing */
            match self.quiet {
                0 => { println!("Starting sort ..."); }
                1 => { println!("Trial {}", trial_number); }
                _ => {}
            }

            timer.start();
            let sorted = sort(vals);
            timer.end();

            match self.quiet {
                0 => { println!("Sort finished."); }
                _ => {}
            }

            if self.verify {
                /* Check that it actually is sorted */
                match self.quiet {
                    0 => { println!("Verifying sort ..."); }
                    _ => {}
                }
                if !ensure_sorted(sorted.as_slice()) {
                    /* Print the values so we can see what they actually look like.
                       Note: Should probably only do this if the array is small */
                    for v in sorted.iter() {
                        println!("{}", *v as uint);
                    }
                    panic!(format!("Trial {}: Array was not sorted correctly", trial_number));
                }
                match self.quiet {
                    0 => { println!("Sort was correct."); }
                    _ => {}
                }
            }

            /* Show the time it took */
            match self.quiet {
                0 => { timer.show_time(); }
                _ => {}
            }
            /* Record the time it took */
            sort_times.push(timer.get_total_time());
        }

        if self.num_trials > 0 {
            /* Print out the average time at the end */
            let total_time = sort_times.iter().map(|&x| x).sum();
            let average_time = total_time / (self.num_trials as u64);
            println!("Average time: {}", timer::format_as_time(average_time));
        }
    }
}

pub fn generate_random_array(size: uint) -> Vec<uint> {
    let mut ret = Vec::with_capacity(size);
    for _ in range(0, size) {
        ret.push(random());
    }
    ret
}

fn ensure_sorted(arr: &[uint]) -> bool {
    let mut previous_value = 0;
    for v in arr.iter() {
        if *v < previous_value {
            return false
        }
        previous_value = *v;
    }
    true
}

