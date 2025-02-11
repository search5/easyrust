fn main() {
    let mut my_string = String::from("Age: 20 Height: 194 Weight: 80");
    my_string.retain(|character| character.is_alphabetic() || character == ' ');    // 문자나 공백이 있다면 유지합니다.
    dbg!(my_string); // 이번에는 println! 대신 dbg!()를 사용해 봅시다.
}
