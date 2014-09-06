/* Sample insertion sort program in Rust.
   Tested to compile with rust-0.6.
 */

extern crate sorting;
use sorting::Benchmark;

fn insertion_sort<T:Ord+Clone>(arr: Vec<T>) -> Vec<T> {
    let mut index = 0;
    let length = arr.len();
    let mut result = arr.clone();

    while index < length {
        let value_to_insert = result[index].clone();
        let mut hole_pos = index;

        while hole_pos > 0 && value_to_insert < result[hole_pos - 1] {
            result.as_mut_slice().swap(hole_pos, hole_pos - 1);
            hole_pos -= 1;
        }
        index += 1;
    }

    return result
}

fn main() {
    let mut bench = Benchmark::new();
    bench.run(insertion_sort);
}

