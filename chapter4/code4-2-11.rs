fn check_if_five(number: i32) -> Result<i32, String> {
    match number {
        5 => Ok(number),
        _ => Err("Sorry, the number wasn't five.".to_string()), // 오류 메시지입니다.
    }
}

fn main() {
    let mut result_vec = Vec::new(); // 결과에 대한 새 벡터를 만듭니다.
    for number in 2..7 {
        result_vec.push(check_if_five(number)); // 각 결과를 벡터에 밀어 넣습니다.
    }
    println!("{:?}", result_vec);
}
