use std::mem;

fn main() {
    println!("{}", mem::size_of::<i32>());
    
    let my_array = [8; 50];
    println!("{}", mem::size_of_val(&my_array));

    let mut some_string = String::from("You can drop a String because it's on the heap");
    mem::drop(some_string);
    // some_string.clear(); 이렇게 하면 패닉에 빠질 것입니다.
}
