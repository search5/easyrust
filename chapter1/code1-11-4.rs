fn times_two(number: i32) -> i32 {
    number * 2
}

fn main() {
    // 섀도잉 없이 러스트를 사용하는 척합니다.
    let final_number = {
        let y = 10;
        let x = 9; // x는 9에서 시작합니다.
        let x_twice = times_two(x); // x의 두 번째 이름
        let x_twice_and_y = x_twice + y; // x의 세 번째 이름
        x_twice_and_y // 섀도잉을 했다면 그냥 x를 사용할 수
        // 있었을텐데 안타깝습니다.
    };
    println!("The number is now: {}", final_number)
}
