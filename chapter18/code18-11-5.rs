use std::time::Instant;

fn bad_random_number(digits: usize) {
    if digits > 9 {
        panic!("Random number can only be up to 9 digits");
    }

    let now = Instant::now();
    let output = format!("{:?}", now);
    output
        .chars()
        .rev()
        .skip(2)
        .take(digits)
        .for_each(|character| print!("{}", character));
    println!();
}

fn main() {
    bad_random_number(1);
    bad_random_number(1);
    bad_random_number(3);
    bad_random_number(3);
}
