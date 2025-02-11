fn main() {
    let my_float: f32 = 5.0;
    let my_other_float = 8.5; // 보통 러스트는 f64를 선택합니다.
    // 그러나 이제 f32에 더해야 함을 압니다.
    // 따라서 my_other_float도 f32를 선택합니다.
    let third_float = my_float + my_other_float;
}
