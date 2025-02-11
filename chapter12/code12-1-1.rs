fn main() {
    prints_number(56);
}

fn prints_number(input: i32) {
    assert_eq!(input % 2, 0); // 두 숫자가 같아야 합니다.
    // number % 2의 결과가 0이 아니면 패닉을 발생시킵니다.
    println!("The number is not odd. It is {input}");
}
