/* Sample bubble sort program in Rust.
   Tested to compile with rust-0.6.
*/
extern crate benchmark;

use benchmark::Benchmark;

fn bubble_sort<T:Ord+Clone>(arr: Vec<T>) -> Vec<T> {
    let mut result = arr.clone();
    let mut left: uint;
    let mut right: uint = arr.len() - 1;
    let mut swap_occurred = true;

    while swap_occurred {
        swap_occurred = false;
        left = 0;
        while left < right {
            if result.get(left+1) < result.get(left) {
                // swap the two values
                result.as_mut_slice().swap(left, left+1);
                swap_occurred = true;
            }
            left += 1;
        }

        right -= 1;
    }

    result
}

fn main() {
    let mut bench = Benchmark::new();
    bench.run(bubble_sort);
}

