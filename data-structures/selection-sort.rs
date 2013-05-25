/* Sample selection sort program in Rust.
   Tested to compile with rust-0.6.
*/
extern mod extra;
extern mod benchmark;
use std::vec;
use benchmark::Benchmark;

fn selection_sort<T:Ord>(arr: &mut [T]) {
    let mut left: uint = 0;
    let mut right: uint;
    let max = arr.len() - 1;
    let mut indexOfMinValue: uint;

    while left < max {
        indexOfMinValue = left;
        right = left + 1;
        while right < max + 1 {
            if arr[right] < arr[indexOfMinValue] {
                indexOfMinValue = right;
            }
            right += 1;
        }

        if indexOfMinValue != left {
            // swap the two values
            vec::swap(arr, left, indexOfMinValue);
        }

        left += 1;
    }

    return;
}

fn main() {
    let mut bench = Benchmark::new();
    bench.run(selection_sort);
}


