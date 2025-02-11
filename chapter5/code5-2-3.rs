fn main() {
    let failure = "Not a number".parse::<i32>();
    failure.rbrbrb(); // ⚠ 컴파일러: 'rbrbrb()가 무엇인가요???'
}
