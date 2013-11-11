#[link(name = "benchmark", vers = "0.1")];
#[crate_type = "lib"];

extern mod extra;
extern mod timer;
use std::{result, os};
use std::rt::io;
use std::rand;
use std::vec;
use std::iter::AdditiveIterator;
use extra::getopts::{getopts, optflag, optflagmulti, optopt};
use timer::Timer;

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
            let opts = ~[
                optflagmulti("q"),
                optflag("quiet"),
                optopt("trialsize"),
                optopt("numtrials"),
                optflag("verify")
                ];
            let matches = match getopts(args.tail(), opts) {
                result::Ok(m) => { m }
                result::Err(f) => { fail!(f.to_str()) }
            };
            if matches.opt_present("q") || matches.opt_present("quiet") {
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
                    match from_str::<uint>(size) {
                        Some(ts) => { self.trial_size = ts }
                        None => { fail!("Trial size must be an integer") }
                    }
                }
                None => {}
            }

            match matches.opt_str("numtrials") {
                Some(trials) => {
                    match from_str::<uint>(trials) {
                        Some(t) => { self.num_trials = t }
                        None => { fail!("Number of trials must be an integer") }
                    }
                }
                None => {}
            }

            self.parse_args = false;
        }
    }

    pub fn run(&mut self, sort: ~fn(~[uint])->~[uint]) {
        self.parse_opts();
        let mut timer = Timer::new();
        let mut sort_times = vec::from_elem(self.num_trials, 0u64);

        for trial_number in range(0, self.num_trials) {
            let vals = generate_random_array(self.trial_size);
            /* Run the sort and record the timing */
            match self.quiet {
                0 => { println("Starting sort ..."); }
                1 => { println!("Trial {}", trial_number); }
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
                    0 => { println("Verifying sort ..."); }
                    _ => {}
                }
                if !ensure_sorted(sorted) {
                    /* Print the values so we can see what they actually look like.
                       Note: Should probably only do this if the array is small */
                    for v in sorted.iter() {
                        println!("{}", *v as uint);
                    }
                    fail!(format!("Trial {}: Array was not sorted correctly", trial_number));
                }
                match self.quiet {
                    0 => { println("Sort was correct."); }
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
        }

        /* Print out the average time at the end */
        let total_time = sort_times.iter().map(|&x| x).sum(); //do iter::sum |f| { sort_times.iter().advance(f) };
        let average_time = total_time / (self.num_trials as u64);
        println!("Average time: {}", timer::format_as_time(average_time));
    }
}

pub fn generate_random_array(size: uint) -> ~[uint] {
    let ret = vec::build(Some(size), 
                    |push| {
                        do size.times {
                            push(rand::random());
                        }
                    }
                    );

    return ret;
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

