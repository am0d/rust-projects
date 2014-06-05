/* Sample insertion sort program in Rust.
   Tested to compile with rust-0.6.
 */
extern crate benchmark;

use benchmark::Benchmark;

fn insertion_sort<T:Ord+Clone>(arr: Vec<T>) -> Vec<T> {
    let mut index = 0;
    let length = arr.len();
    let mut result = arr.clone();

    while index < length {
        let valueToInsert = result.get(index).clone();
        let mut holePos = index;

        while holePos > 0 && valueToInsert < *result.get(holePos - 1) {
            result.as_mut_slice().swap(holePos, holePos - 1);
            holePos -= 1;
        }
        index += 1;
    }

    return result
}

fn main() {
    let mut bench = Benchmark::new();
    bench.run(insertion_sort);
}

