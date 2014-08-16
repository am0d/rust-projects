/* Sample selection sort program in Rust.
   Tested to compile with rust-0.6.
*/

extern crate sorting;
use sorting::Benchmark;

fn selection_sort<T:Ord+Clone>(arr: Vec<T>) -> Vec<T> {
    let mut left: uint = 0;
    let mut right: uint;
    let mut result =  arr.clone();
    let max = result.len() - 1;
    let mut indexOfMinValue: uint;

    while left < max {
        indexOfMinValue = left;
        right = left + 1;
        while right < max + 1 {
            if result[right] < result[indexOfMinValue] {
                indexOfMinValue = right;
            }
            right += 1;
        }

        if indexOfMinValue != left {
            // swap the two values
            result.as_mut_slice().swap(left, indexOfMinValue);
        }

        left += 1;
    }

    result
}

fn main() {
    let mut bench = Benchmark::new();
    bench.run(selection_sort);
}


