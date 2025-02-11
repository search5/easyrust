fn main() {
    let mut number = 10;
    let number_change = &mut number; // 변경 가능한 참조 생성
    *number_change += 10; // 불변 참조를 사용해 10을 더함
    let number_ref = &number; // 변경 불가능한 참조 생성
    println!("{}", number_ref); // 불변 참조 출력
}
