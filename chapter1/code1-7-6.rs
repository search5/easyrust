fn main() {
    let my_number = {
        let second_number = 8;
        second_number + 9 // 세미콜론이 없으므로 코드 블록은 8 + 9를
        // 반환합니다. 함수처럼 동작합니다.
    };
    println!("My number is: {}", my_number);
}
