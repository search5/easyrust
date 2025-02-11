// 두 개의 i32 타입 number_one과 number_two가 함수에 들어갑니다.
fn multiply(number_one: i32, number_two: i32) {
    let result = number_one * number_two;
    println!(“{} times {} is {}”, number_one, number_two, result);
}

fn main() {
    multiply(8, 9); // 숫자를 직접 전달할 수 있습니다.
    let some_number = 10; // 또는 두 개의 변수를 선언한 다음,
    let some_other_number = 2;
    multiply(some_number, some_other_number); // 함수에 변수를 넣을 수 있습니다.
}
