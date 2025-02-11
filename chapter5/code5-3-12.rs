fn try_two_unwraps(input: Vec<Option<i32>>) {
    println!("Index 0 is: {}", input[0].unwrap());
    println!("Index 1 is: {}", input[1].unwrap());
}

fn main() {
    let vector = vec![None, Some(1000)]; // 이 벡터에는 None이 있으므로
                                         // 패닉 상태가 됩니다.
    try_two_unwraps(vector);
}
