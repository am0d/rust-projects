/* Sample swap sort program in Rust.
   Tested to compile with rust-0.6-f1ddb8d.
*/
extern crate benchmark;

use benchmark::Benchmark;

fn swap_sort<T:Ord+Clone>(arr: Vec<T>) -> Vec<T> {
    let mut left = 0;
    let mut right: uint;
    let mut result = arr.clone();
    let max = result.len();

    while left < max {
        right = left + 1;
        while right < max {
            if result.get(right) < result.get(left) {
                // swap the two values
                //vec::swap(result, left, right);
                result.as_mut_slice().swap(left, right);
            }
            right += 1;
        }

        left += 1;
    }

    result
}

fn main() {
    let mut bench = Benchmark::new();
    bench.run(swap_sort);
}
