fn main() {
    let my_float: f64 = 5.0;
    let my_other_float: f32 = 8.5;
    // my_other_float as f64는 my_other_float를 f64처럼 사용합니다.
    let third_float = my_float + my_other_float as f64;
}
