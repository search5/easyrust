fn main() {
    let mut num_vec = Vec::new();
    println!("{}", num_vec.capacity()); // 0개 요소: 0을 출력
    num_vec.push('a'); // 한 개 문자 추가
    println!("{}", num_vec.capacity()); // 1개 요소: 4를 출력
    // 항목이 1개인 벡터는 항상 용량이 4로 시작합니다.
    num_vec.push('a'); // 하나 더 추가
    num_vec.push('a'); // 하나 더 추가
    num_vec.push('a'); // 하나 더 추가
    println!("{}", num_vec.capacity()); // 4개 요소: 여전히 4를 출력
    num_vec.push('a'); // 하나 더 추가
    println!("{}", num_vec.capacity()); // 8을 출력
    // 요소가 5개지만 벡터의 추가 공간을 만들기 위해 4에서 8로 용량을 2배 늘렸습니다.
}
