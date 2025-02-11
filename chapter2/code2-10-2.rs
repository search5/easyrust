fn change_number(counter: i32) -> i32 {
    counter + 10
}

fn main() {
    let my_number;
    {
        // 복잡한 코드를 쓰고 있다고 가정하기 위해 코드 블록이 필요한 척합니다.
        let number = {
            // 여기에 숫자를 만드는 코드가 있다고 가정합니다.
            // 많은 코드가 나온 후 마지막으로 다음이 나옵니다.
            57
        };
        my_number = change_number(number);
    }
    println!("{my_number}");
}
