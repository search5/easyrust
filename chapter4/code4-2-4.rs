fn take_fifth(value: Vec<i32>) -> Option<i32> {
    if value.len() < 5 {
        None
    } else {
        Some(value[4])
    }
}

fn handle_option(my_option: Vec<Option<i32>>) {
    for item in my_option {
        match item {
            Some(number) => println!("Found a {number}!"),
            None => println!("Found a None!"),
        }
    }
}

fn main() {
    let new_vec = vec![1, 2];
    let bigger_vec = vec![1, 2, 3, 4, 5];
    let mut option_vec = Vec::new();
    // Option을 보유할 새 벡터를 만드세요. 벡터 타입은 Vec<Option<i32>>입니다.
    // 이는 Option<i32>를 보유한 벡터를 의미합니다.

    option_vec.push(take_fifth(new_vec)); // "None"을 벡터에 넣습니다.
    option_vec.push(take_fifth(bigger_vec));
    // 이렇게 하면 "Some(5)"가 벡터에 밀어 넣어집니다.
    
    handle_option(option_vec);
    // handle_option은 벡터의 모든 옵션을 살펴봅니다.
    // Some이면 값을 출력합니다. None이면 Option에 접근하지 않습니다.
}
