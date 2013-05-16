/* Sample bubble sort program in Rust.
   Tested to compile with rust-0.6-f1ddb8d.

   $ rustc bubble-sort.rs
   $ time ./bubble-sort 

   real 1m2.749s
   user 1m2.580s
   sys  0m0.008s
*/
extern mod std;
use core::rand;
use core::vec;
use core::cell;

fn generate_random_array(size: uint) -> ~[uint] {
    let ret = vec::build_sized(size, |push| {
        for size.times {
            push(rand::random());
        }
    });

    return ret;
}

struct Timer {
    mut start_time: u64,
    mut end_time: u64
}

impl Timer {
    fn new() -> Timer {
        Timer { start_time: 0, end_time: 0}
    }
    fn start(&mut self) -> () {
        self.start_time = std::time::precise_time_ns();
    }
    fn end(&mut self) -> () {
        self.end_time = std::time::precise_time_ns();
    }
    fn get_time_string(&mut self) -> ~str {
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
            time_string += fmt!("%2?.", seconds);
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
    fn show_time(&mut self) -> () {
        core::io::println(fmt!("Total time: %s", self.get_time_string()));
    }
    fn run(&mut self, operation: &fn()) -> () {
        self.start();
        operation();
        self.end();
    }
}

fn bubble_sort(arr: &mut [uint]) -> () {
    let mut left = 0;
    let mut right: uint;
    let mut modified: bool;
    let max = arr.len();
    while left < max {
        right = left + 1;
        modified = false;
        while right < max {
            if arr[right] < arr[left] {
                // swap the two values
                let old_right = arr[right];
                arr[right] = arr[left];
                arr[left] = old_right;
                modified = true;
            }
            right += 1;
        }
        if !modified {
            // early exit if we have everything in place already
            return;
        }

        left += 1;
    }

    return;
}

fn ensure_sorted(arr: &[uint]) -> bool {
    let mut left = 0;
    let mut previous_value = 0;
    let max = arr.len();
    while left < max {
        if arr[left] < previous_value {
            return false
        }
        previous_value = arr[left];
        left += 1;
    }
    true
}
    

fn run(num: uint) -> () {
    let mut vals = generate_random_array(num);
    let sorted_vals:cell::Cell<~[uint]> = cell::Cell(~[]);
    let mut timer = Timer::new();

    let runner = || {
        bubble_sort(vals);
    };

    /* Run the sort and record the timing */
    core::io::println("Starting sort ...");
    timer.run(runner);
    core::io::println("Sort finished, verifying ...");

    /* Check that it actually is sorted */
    if !ensure_sorted(vals) {
        /* Print the values so we can see what they actually look like.
           Note: Should probably only do this if the array is small */
        for sorted_vals.take().each |v| {
            core::io::println(fmt!("%?", v));
        }
        fail!("Array was not sorted correctly");
    }

    /* Show the time it took */
    core::io::println("Sort was correct.");
    timer.show_time();
}

fn main() {
    run(100000);
}
