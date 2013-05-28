/* Sample merge sort program in Rust.
   Tested to compile with rust-0.6-f1ddb8d.
*/
extern mod extra;
extern mod benchmark;
use std::vec;
use std::comm::{stream, Chan, Port};
use std::cell::Cell;
use benchmark::Benchmark;

fn merge_sort<T:Ord+Copy+Owned>(arr: ~[T]) -> ~[T] {
    let length = arr.len();
    if length <= 1 {
        return arr.to_owned();
    }

    let middle = length / 2;
    let mut left: ~[T] = vec::from_elem(middle, copy arr[0]);
    let mut right: ~[T] = vec::from_elem(length - middle, copy arr[0]);
    let mut index = 0;


    while index < middle {
        left[index] = arr[index];
        index += 1;
    }

    while index < length {
        right[index - middle] = arr[index];
        index += 1;
    }

    let (port, chan): (Port<~[T]>, Chan<~[T]>) = stream();
    let left_cell = Cell(left);
    do spawn {
        let sorted_left =  merge_sort(left_cell.take());
        chan.send(sorted_left);
    }
    right = merge_sort(right);

    left = port.recv();

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
    bench.run(merge_sort);
}

