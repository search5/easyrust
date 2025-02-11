fn take_fifth(value: Vec<i32>) -> Option<i32> {
    if value.len() < 5 {
        // .len()은 벡터의 길이를 제공합니다. 길이가 5 이상이어야 합니다.
        None
    } else {
        Some(value[4])
    }
}

fn main() {
    let new_vec = vec![1, 2];
    let bigger_vec = vec![1, 2, 3, 4, 5];
    println!("{:?}, {:?}", take_fifth(new_vec), take_fifth(bigger_vec));
}
