fn multiply(number_one: i32, number_two: i32) -> i32 {
    let result = number_one * number_two;
    println!("{} times {} is {}", number_one, number_two, result);
    result // 함수에서 반환하는 i32입니다.
}

fn main() {
    // multiply()를 사용해 화면에 출력하고 결과를 multiply_result에 전달합니다.
    let multiply_result = multiply(8, 9);
}
