fn main() {
    let my_vec = vec![8.0_f64, 7.6, 9.4, 10.0, 22.0, 77.345, 10.22, 3.2, -7.77, -10.0];
    let maximum = my_vec.iter().fold(f64::MIN, |current_number, next_number|current_number.max(*next_number));
    // 참고: f64의 가장 낮은 숫자와 비교를 시작합니다.

    let minimum = my_vec.iter().fold(f64::MAX, |current_number, next_number|current_number.min(*next_number));
    // 반대로 f64의 가장 높은 숫자와 비교를 시작합니다.

    println!("{}, {}", maximum, minimum);
}
