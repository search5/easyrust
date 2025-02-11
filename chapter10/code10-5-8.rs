fn main() {
    let my_vec: Vec<i32> = vec![8, 9, 10];

    let my_closure = || {
        my_vec
            .into_iter() // into_iter가 소유권을 가져갑니다.
            .map(|x| x as u8) // u8 타입으로 바꿉니다.
            .map(|x| x * 2) // 2를 곱합니다.
            .collect::<Vec<u8>>() // Vec으로 수집합니다.
    };

    let new_vec = my_closure();
    println!("{:?}", new_vec);
}
