// ⚠
#![no_implicit_prelude]
fn main() {
    let my_vec = vec![8, 9, 10];
    let my_string = String::from("This won't work");
    println!("{:?}, {}", my_vec, my_string);
}
