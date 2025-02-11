use std::fmt::Display;
use std::cmp::PartialOrd;

fn compare_and_display<T: Display, U: Display + PartialOrd>(statement: T, num_1: U, num_2: U) {
    println!("{statement}! Is {num_1} greater than {num_2}? {}", num_1 > num_2);
}

fn main() {
    compare_and_display("Listen up!", 9, 8);
}
