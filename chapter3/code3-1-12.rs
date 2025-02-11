fn main() {
    let mut num_vec = Vec::with_capacity(8); // 용량을 8로 지정합니다.
    num_vec.push('a'); // 한 문자 추가
    println!("{}", num_vec.capacity()); // 8을 출력
    num_vec.push('a'); // 하나 더 추가
    println!("{}", num_vec.capacity()); // 8을 출력
    num_vec.push('a'); // 하나 더 추가
    println!("{}", num_vec.capacity()); // 8을 출력
    num_vec.push('a'); // 하나 더 추가
    num_vec.push('a'); // 하나 더 추가, 이제 5개의 요소가 있습니다.
    println!("{}", num_vec.capacity()); // 여전히 8을 출력합니다.
}
