/* Sample merge sort program in Rust.
   Tested to compile with rust-0.6-f1ddb8d.
*/
extern crate benchmark;
use std::vec;
use std::comm::channel;
use std::cell::RefCell;
use benchmark::Benchmark;

fn parallel_merge_sort_helper<T:Ord+Clone+Send>(arr: ~[T]) -> ~[T] {
    let max_threads = ::std::rt::default_sched_threads();

    parallel_merge_sort(arr, 0, max_threads)
}

fn parallel_merge_sort<T:Ord+Clone+Send>(arr: ~[T], depth: uint, max_threads: uint) -> ~[T] {
    let length = arr.len();
    if length <= 1 {
        return arr.to_owned();
    }

    let middle = length / 2;
    let mut left = arr.slice(0, middle).to_owned();
    let mut right = arr.slice(middle, length).to_owned();

    if depth < max_threads {
        /* Create channel to pass the results back */
        let (sender, receiver) = channel();
        let left_cell = RefCell::new(left); // the only way to access the above mutable field
        spawn(proc() {
            // take the ref out of the cell, sort it, and send it back to the parent process
            let sorted_left =  parallel_merge_sort(left_cell.get(), depth + 1, max_threads);
            sender.send(sorted_left);
        });
        right = parallel_merge_sort(right, depth + 1, max_threads);

        left = receiver.recv();
    } else {
        left = parallel_merge_sort(left, depth, max_threads);
        right = parallel_merge_sort(right, depth, max_threads);
    }

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
    bench.run(parallel_merge_sort_helper);
}

