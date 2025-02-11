fn gives_higher_i32(one: i32, two: i32) {
    let higher = if one > two { one } else { two };
    println!("{higher} is higher.");
}

fn main() {
    gives_higher_i32(8, 10);
}
