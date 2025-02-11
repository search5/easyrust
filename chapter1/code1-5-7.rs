fn main() {
    let my_float: f64 = 5.0; // 컴파일러는 f64를 봅니다.
    let my_other_float: f32 = 8.5; // 컴파일러는 f32를 봅니다. f64와는 다른 타입입니다.
    let third_float = my_float + // my_float를 뭔가에 추가하려고 하므로
    // f64에 다른 f64를 더한 것이어야 합니다. 이제 f64를 기대합니다.
    let third_float = my_float + my_other_float; // 하지만 f32를 찾았습니다.
    // 더할 수 없습니다.
}