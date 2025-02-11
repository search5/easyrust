fn main() {
    let my_vec: Vec<u8> = [1, 2, 3].into();
    let my_vec2: Vec<_> = [9, 0, 10].into();
    // Vec<_>은 '나를 위한 Vec 타입 선택'을 의미합니다.
    // 러스트는 Vec<i32>를 선택할 것입니다.
}
