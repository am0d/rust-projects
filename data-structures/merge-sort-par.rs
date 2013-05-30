/* Sample merge sort program in Rust.
   Tested to compile with rust-0.6-f1ddb8d.
*/
extern mod extra;
extern mod benchmark;
use std::vec;
use std::comm::{stream, Chan, Port};
use std::cell::Cell;
use benchmark::Benchmark;

fn parallel_merge_sort_helper<T:Ord+Copy+Owned>(arr: ~[T]) -> ~[T] {
    parallel_merge_sort(arr, 0)
}

fn parallel_merge_sort<T:Ord+Copy+Owned>(arr: ~[T], depth: uint) -> ~[T] {
    let length = arr.len();
    if length <= 1 {
        return arr.to_owned();
    }

    let middle = length / 2;
    let mut left = vec::to_owned(vec::slice(arr, 0, middle));
    let mut right = vec::to_owned(vec::slice(arr, middle, length));

    if depth < 8 {
        /* Create channel to pass the results back */
        let (port, chan): (Port<~[T]>, Chan<~[T]>) = stream();
        let left_cell = Cell(left); // the only way to access the above mutable field
        do spawn {
            // take the ref out of the cell, sort it, and send it back to the parent process
            let sorted_left =  parallel_merge_sort(left_cell.take(), depth + 1);
            chan.send(sorted_left);
        }
        right = parallel_merge_sort(right, depth + 1);

        left = port.recv();
    } else {
        left = parallel_merge_sort(left, depth);
        right = parallel_merge_sort(right, depth);
    }

    merge(left, right)
}

fn merge<T:Ord+Copy>(left_orig: ~[T], right_orig: ~[T]) -> ~[T] {
    let mut left = copy left_orig;
    let mut right = copy right_orig;
    let mut result = vec::from_elem(0, copy left[0]);

    while left.len() > 0 || right.len() > 0 {
        if left.len() > 0 && right.len() > 0 {
            if left[0] < right[0] {
                result.push(left.shift());
            }
            else {
                result.push(right.shift());
            }
        }
        else if left.len() > 0 {
            result.push(left.shift());
        }
        else {
            result.push(right.shift());
        }
    }
    
    return result;
}

fn main() {
    let mut bench = Benchmark::new();
    bench.run(parallel_merge_sort_helper);
}

