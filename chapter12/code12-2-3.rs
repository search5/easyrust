#![allow(dead_code)]
#![allow(unused_variables)]
struct Struct1 {} // 다섯 개의 구조체를 생성합니다.
struct Struct2 {}
struct Struct3 {}
struct Struct4 {}
struct Struct5 {}

fn main() {
    // 네 개의 변수입니다. 우리는 이들을 사용하지 않지만,
    // 컴파일러는 조용하게 지나갑니다.
    let char1 = 'ん';
    let char2 = ';';
    let some_str = "I'm just a regular &str";
    let some_vec = vec!["I", "am", "just", "a", "vec"];
}
