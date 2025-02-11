fn main() {
    let my_vec = vec![2, 3, 4];
    let get_one = my_vec.get(0); // 0은 벡터의 첫 번째 값인 숫자를 얻습니다.
    let get_two = my_vec.get(10); // None을 반환합니다.
    println!("{:?}", get_one);
    println!("{:?}", get_two);
}
