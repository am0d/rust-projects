/* Sample swap sort program in Rust.
   Tested to compile with rust-0.6-f1ddb8d.
*/
extern mod extra;
extern mod benchmark;
use std::vec;
use benchmark::Benchmark;

fn swap_sort<T:Ord+Copy>(arr: ~[T]) -> ~[T] {
    let mut left = 0;
    let mut right: uint;
    let mut result = copy arr;
    let max = result.len();

    while left < max {
        right = left + 1;
        while right < max {
            if result[right] < result[left] {
                // swap the two values
                vec::swap(result, left, right);
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
