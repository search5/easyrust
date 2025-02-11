// ⚠
fn take_fifth(value: Vec<i32>) -> Option<i32> {
    if value.len() < 5 { None } else { Some(value[4]) }
}

fn main() {
    let new_vec = vec![1, 2];
    let bigger_vec = vec![1, 2, 3, 4, 5];
    println!(
        "{:?}, {:?}",
        take_fifth(new_vec).unwrap(), // 이것은 None입니다.
                                                           // .unwrap()은 패닉에 빠질 것입니다!
        take_fifth(bigger_vec).unwrap()
    );
}
