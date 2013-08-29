/* Sample insertion sort program in Rust.
   Tested to compile with rust-0.6.
 */
extern mod extra;
extern mod benchmark;

use benchmark::Benchmark;

fn insertion_sort<T:Ord+Clone>(arr: ~[T]) -> ~[T] {
    let mut index = 0;
    let length = arr.len();
    let mut result = arr.clone();

    while index < length {
        let valueToInsert = result[index].clone();
        let mut holePos = index;

        while holePos > 0 && valueToInsert < result[holePos - 1] {
            result.swap(holePos, holePos - 1);
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

