fn main() {
    let my_vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let new_vec = my_vec
        .into_iter() // 항목을 반복(내부의 각 항목에 대해 작업)합니다.
                     // into_iter()는 참조가 아닌 소유한 값을 제공합니다.
        .skip(3) // 3개 항목(0, 1, 2)을 건너뛰고
        .take(4) // 4개 항목(3, 4, 5, 6)을 가져옵니다.
        .collect::<Vec<i32>>(); // 새 Vec<i32>에 넣습니다.
    println!("{new_vec:?}");
}
