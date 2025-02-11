use std::fmt::Display;
// 이를 출력하는 일반적인 기능을 만들 것이므로 Display가 필요합니다.

fn print_vec<T: Display>(input: &Vec<T>) {
    // 타입 T가 Display를 구현하는 경우 인자로 Vec<T>를 취하세요.
    for item in input {
        print!("{item} ");
    }
    println!();
}

fn main() {
    let array_vec = Vec::from([8, 9, 10]); // 배열에서 Vec을 만듭니다.
    print_vec(&array_vec);

    let str_vec = Vec::from("What kind of vec will I be?");
    // &str을 벡터로 만든다고요? 재미있겠네요.
    print_vec(&str_vec);
    
    let string_vec = Vec::from("What kind of vec will a String be?".to_string());
    // 또한 문자열에서 Vec을 만듭니다.
    print_vec(&string_vec);
}
