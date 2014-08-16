/* Sample merge sort program in Rust.
   Tested to compile with rust-0.6-f1ddb8d.
*/

extern crate sorting;
use sorting::Benchmark;

fn merge_sort<T:Ord+Clone>(arr: Vec<T>) -> Vec<T> {
    let length = arr.len();
    if length <= 1 {
        return arr;
    }

    let middle = length / 2;
    let mut left = Vec::from_slice(arr.slice(0, middle));
    let mut right = Vec::from_slice(arr.slice(middle, length));

    left = merge_sort(left);
    right = merge_sort(right);

    merge(left, right)
}

fn merge<T:Ord+Clone>(left_orig: Vec<T>, right_orig: Vec<T>) -> Vec<T> {
    let mut left = left_orig.clone();
    let mut right = right_orig.clone();
    let mut result = Vec::with_capacity(left_orig.len() + right_orig.len());
    result.push(left[0].clone());

    while left.len() > 0 || right.len() > 0 {
        if left.len() > 0 && right.len() > 0 {
            if left[0] < right[0] {
                result.push(left.remove(0).unwrap());
            }
            else {
                result.push(right.remove(0).unwrap());
            }
        }
        else if left.len() > 0 {
            result.push(left.remove(0).unwrap());
        }
        else {
            result.push(right.remove(0).unwrap());
        }
    }
    
    return result;
}

fn main() {
    let mut bench = Benchmark::new();
    bench.run(merge_sort);
}

