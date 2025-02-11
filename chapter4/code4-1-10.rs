use std::cmp::PartialOrd;
use std::fmt::Display;

fn compare_and_display<T, U>(statement: T, num_1: U, num_2: U) where
    T: Display,
    U: Display + PartialOrd
{
    println!("{statement}! Is {num_1} greater than {num_2}? {}", num_1 > num_2);
}

fn main() {
    compare_and_display("Listen up!", 9, 8);
}
