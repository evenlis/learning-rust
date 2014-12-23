use std::thread::Thread;

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
    let thread1 = Thread::spawn(move || {
        println!("{}", stupid_fib(42));
    });
    let thread2 = Thread::spawn(move || {
        println!("{}", stupid_fib(42));
    });
    let thread3 = Thread::spawn(move || {
        println!("{}", stupid_fib(42));
    });
    let thread4 = Thread::spawn(move || {
        println!("{}", stupid_fib(42));
    });
}
