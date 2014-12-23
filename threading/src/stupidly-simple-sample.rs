use std::thread::Thread;
use std::io::{File,BufferedReader};
use std::time;

// Runs at about 8 seconds single-thread, should suffice to measure the impact of multiple threads
fn stupid_fib(n: int) -> int {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }
    stupid_fib(n-1) + stupid_fib(n-2)
}

fn main() {
    println!("{}", stupid_fib(42));
}
