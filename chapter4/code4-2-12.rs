// ⚠
fn main() {
    let error_value: Result<i32, &str> = Err("There was an error");
    // 이미 오류가 있는 Result를 만듭니다.
    println!("{}", error_value.unwrap()); // 값을 풀어냅니다.
}
