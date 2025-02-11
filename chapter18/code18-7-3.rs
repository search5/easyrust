fn main() {
    let mut push_string = String::with_capacity(4587520);
    let mut capacity_counter = 0;

    for _ in 0..100_000 {
        if push_string.capacity() != capacity_counter {
            println!("{}", push_string.capacity());
            capacity_counter = push_string.capacity();
        }

        push_string.push_str("I'm getting pushed into the string!");
    }

    push_string.shrink_to_fit();
    println!("{}", push_string.capacity());

    push_string.push('a');
    println!("{}", push_string.capacity());
    
    push_string.shrink_to_fit();
    println!("{}", push_string.capacity());
}
