/* Sample insertion sort program in Rust.
   Tested to compile with rust-0.6.
 */
extern mod extra;
extern mod benchmark;
use std::vec;
use benchmark::Benchmark;

fn insertion_sort<T:Ord+Copy>(arr: ~[T]) -> ~[T] {
    let mut index = 0;
    let length = arr.len();
    let mut result = copy arr;

    while index < length {
        let valueToInsert = copy result[index];
        let mut holePos = index;

        while holePos > 0 && valueToInsert < result[holePos - 1] {
            vec::swap(result, holePos, holePos - 1);
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

