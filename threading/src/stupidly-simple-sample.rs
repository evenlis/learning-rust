use std::thread::Thread;
use std::any::Any;

// Runs at about 8 seconds single-thread, should suffice to measure the impact of multiple threads
// And yes, I know this is a moronic implementation, that's the entire point
fn stupid_fib(n: int) -> int {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }
    stupid_fib(n-1) + stupid_fib(n-2)
}

fn main() {
    let result1 = Thread::spawn(move || {
        return stupid_fib(42);
    });
    let result2 = Thread::spawn(move || {
        return stupid_fib(42);
    });
    let result3 = Thread::spawn(move || {
        return stupid_fib(42);
    });
    let result4 = Thread::spawn(move || {
        return stupid_fib(42);
    });
    let mut sum = 0i;
    match result1.join() {
        Ok(x) => {sum += x}
        Err(e) => {println!("Error")}
    }
    match result2.join() {
        Ok(x) => {sum += x}
        Err(e) => {println!("Error")}
    }
    match result3.join() {
        Ok(x) => {sum += x}
        Err(e) => {println!("Error")}
    }
    match result4.join() {
        Ok(x) => {sum += x}
        Err(e) => {println!("Error")}
    }
    println!("4 * fib(42) = {}", sum);
}
