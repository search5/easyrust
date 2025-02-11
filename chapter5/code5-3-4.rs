fn prints_three_things(vector: Vec<i32>) {
    println!("{}, {}, {}", vector[0], vector[1], vector[2]);
}

fn main() {
    let my_vec = vec![8, 9, 10, 10, 55, 99]; // 이제 my_vec에는 항목이 6개 있습니다.
    prints_three_things(my_vec);
}
