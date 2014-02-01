/* Sample merge sort program in Rust.
   Tested to compile with rust-0.6-f1ddb8d.
*/
extern mod extra;
extern mod benchmark;
use std::vec;
use benchmark::Benchmark;

fn merge_sort<T:Ord+Clone>(arr: ~[T]) -> ~[T] {
    let length = arr.len();
    if length <= 1 {
        return arr.to_owned();
    }

    let middle = length / 2;
    let mut left = arr.slice(0, middle).to_owned();
    let mut right = arr.slice(middle, length).to_owned();

    left = merge_sort(left);
    right = merge_sort(right);

    merge(left, right)
}

fn merge<T:Ord+Clone>(left_orig: ~[T], right_orig: ~[T]) -> ~[T] {
    let mut left = left_orig.clone();
    let mut right = right_orig.clone();
    let mut result = vec::from_elem(0, left[0].clone());

    while left.len() > 0 || right.len() > 0 {
        if left.len() > 0 && right.len() > 0 {
            if left[0] < right[0] {
                result.push(left.shift().unwrap());
            }
            else {
                result.push(right.shift().unwrap());
            }
        }
        else if left.len() > 0 {
            result.push(left.shift().unwrap());
        }
        else {
            result.push(right.shift().unwrap());
        }
    }
    
    return result;
}

fn main() {
    let mut bench = Benchmark::new();
    bench.run(merge_sort);
}

