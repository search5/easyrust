fn main() {
    let num_vec = vec![10, 9, 8];
    
    num_vec
        .iter() // num_vec에 대해 이터레이터를 가져옵니다.
        .enumerate() // (index, number)를 얻습니다.
        .for_each(|(index, number)|
            println!("Index number {} has number {}", index, number)
        ); // 각 항목을 볼 때 뭔가를 합니다.
}
