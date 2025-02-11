enum Number {
    U32(u32),
    I32(i32),
}

fn get_number(input: i32) -> Number {
    let number = match input.is_positive() {
        true => Number::U32(input as u32), // 양수면 숫자를 u32로 변경하세요.
        false => Number::I32(input), // 그렇지 않으면 이미 i32 타입이므로
        // 바로 숫자를 제공하세요.
    };
    number
}

fn main() {
    let my_vec = vec![get_number(-800), get_number(8)];
    for item in my_vec {
        match item {
            Number::U32(number) => println!("It's a u32 with the value {}", number),
            Number::I32(number) => println!("It's an i32 with the value {}", number),
        }
    }
}
