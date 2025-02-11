fn main() {
    let num_vec = vec![2, 4, 6];

    let double_vec = num_vec        // num_vec을 가져옵니다.
        .iter()                     // num_vec의 이터레이터를 가져옵니다.
        .map(|number| number * 2)   // 각 항목에 2를 곱하고 전달합니다.
        .collect::<Vec<i32>>();     // 그런 다음 여기서 새로운 Vec을 만듭니다.
    println!("{:?}", double_vec);
}
