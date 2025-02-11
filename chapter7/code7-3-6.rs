fn main() {
    let my_vec = vec![8, 9, 10];
    
    let fourth = my_vec.get(3).unwrap_or_else(|| {
        // unwrap을 시도하세요. 동작하지 않으면 my_vec에 인덱스 [0]이 있는지
        // 확인합니다.
        if my_vec.get(0).is_some() {
            &my_vec[0] // 뭔가가 있으면 인덱스 0의 숫자를 제공합니다.
        } else {
            &0 // 그렇지 않으면 &0을 제공합니다.
        }
    });

    println!("{fourth}");
}
