fn main() {
    let new_vec = (1..).take(10).collect::<Vec<i32>>();
    // 또는 다음과 같이 쓸 수 있습니다.
    // let new_vec: Vec<i32> = (1..).take(10).collect();
    println!("{new_vec:?}");
}
