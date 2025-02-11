fn times_two(number: i32) -> i32 {
    number * 2
}

fn main() {
    let final_number = {
        let y = 10;
        let x = 9; // x는 9에서 시작합니다.
        let x = times_two(x); // x의 새로운 섀도우 값: 18
        let x = x + y; // x의 새로운 섀도우 값: 28
        x // return x: final_number는 이제 x의 값입니다.
    };
    println!("The number is now: {}", final_number)
}
