use std::mem::transmute;

fn main() {
    let x = -19;
    let y: u32 = unsafe { transmute::<i32, u32>(x) };
    println!("{y}");
}
