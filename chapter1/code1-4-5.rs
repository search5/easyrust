fn main() {
    println!("Size of a char: {}", std::mem::size_of::<char>()); // 4 바이트
    // .len()은 문자열의 크기를 바이트 단위로 제공합니다.
    println!("Size of string containing 'a': {}", "a".len());
    println!("Size of string containing 'ß': {}", "ß".len());
    println!("Size of string containing '国': {}", "国".len());
    println!("Size of string containing '𓅱': {}", "𓅱".len());
}
