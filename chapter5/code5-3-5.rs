fn prints_three_things(vector: Vec<i32>) {
    if vector.len() != 3 {
        panic!("my_vec must always have three items");
        // 길이가 3이 아니면 패닉을 발생시킵니다.
    }
    println!("{}, {}, {}", vector[0], vector[1], vector[2]);
}

fn main() {
    let my_vec = vec![8, 9, 10];
    prints_three_things(my_vec);
}
