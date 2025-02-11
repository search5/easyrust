fn main() {
    // 정확한 숫자를 알고 있으며 다른 큰 숫자도 사용할 수 있습니다.
    let mut push_string = String::with_capacity(4587520);
    let mut push_string = String::with_capacity(4587520);
    let mut capacity_counter = 0;

    for _ in 0..100_000 {
        if push_string.capacity() != capacity_counter {
            println!("{}", push_string.capacity());
            capacity_counter = push_string.capacity();
        }
        
        push_string.push_str("I'm getting pushed into the string!");
    }
}
