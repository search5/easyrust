use std::fmt::{Debug, Display}; // Debug를 추가합니다.

fn print_it<T>(input: T) // 이렇게 작성하면 읽기 쉽습니다.
where
    T: AsRef<str> + Debug + Display, // 트레이트도 읽기 쉬워집니다.
{
    println!("{}", input)
}

fn main() {
    print_it("Please print me");
    print_it("Also, please print me".to_string());
}
