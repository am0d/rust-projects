/* Sample swap sort program in Rust.
   Tested to compile with rust-0.6-f1ddb8d.
*/
extern mod std;
extern mod benchmark;
use core::vec;
use benchmark::Benchmark;

fn swap_sort(arr: &mut [uint]) -> () {
    let mut left = 0;
    let mut right: uint;
    let max = arr.len();

    while left < max {
        right = left + 1;
        while right < max {
            if arr[right] < arr[left] {
                // swap the two values
                vec::swap(arr, left, right);
            }
            right += 1;
        }

        left += 1;
    }

    return;
}

fn main() {
    let mut bench = Benchmark::new();
    bench.run(swap_sort);
}
