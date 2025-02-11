fn main() {
    let mut push_string = String::new();
    let mut capacity_counter = 0; // 용량은 0부터 시작합니다.

    for _ in 0..100_000 { // 이 작업을 100,000번 수행하세요.
        if push_string.capacity() != capacity_counter { // 용량이 다른지 확인하세요.
            println!("{}", push_string.capacity()); // 값이 있으면 출력합니다.
            capacity_counter = push_string.capacity(); // 카운터를 업데이트합니다.
        }

        // 매번 밀어 넣습니다.
        push_string.push_str("I'm getting pushed into the string!");
    }
}
