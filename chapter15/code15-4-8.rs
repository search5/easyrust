use std::mem::transmute;

struct User {
    name: String,
    number: u32,
}

fn main() {
    let some_i32s = [1, 2, 3, 4, 5, 6, 7, 8];
    let user = unsafe { transmute::<[i32; 8], User>(some_i32s) };
}
