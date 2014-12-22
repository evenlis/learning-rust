use std::thread::Thread;
use std::io::{File,BufferedReader};
use std::time;

fn ascii_sum(word: String) -> int {
    let mut sum = 0i;
    for c in word.chars() {
        sum += c as int;
    }
    sum-10
}

fn main() {

}
