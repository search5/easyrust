use std::fmt::Display;

fn gives_higher_i32<T: PartialOrd + Display>(one: T, two: T) {
    let higher = if one > two { one } else { two };
    println!("{higher} is higher.");
}

fn main() {
    gives_higher_i32(8, 10);
}
