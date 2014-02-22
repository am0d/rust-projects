/* Sample swap sort program in Rust.
   Tested to compile with rust-0.6-f1ddb8d.
*/
extern crate extra;
extern crate benchmark;

use benchmark::Benchmark;

fn swap_sort<T:Ord+Clone>(arr: ~[T]) -> ~[T] {
    let mut left = 0;
    let mut right: uint;
    let mut result = arr.clone();
    let max = result.len();

    while left < max {
        right = left + 1;
        while right < max {
            if result[right] < result[left] {
                // swap the two values
                //vec::swap(result, left, right);
                result.swap(left, right);
            }
            right += 1;
        }

        left += 1;
    }

    return result;
}

fn main() {
    let mut bench = Benchmark::new();
    bench.run(swap_sort);
}
