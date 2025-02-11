#[derive(Clone, Copy)] // Copy를 사용하려면 Clone도 필요합니다.
struct NumberAndBool {
    number: i32, // i32는 Copy를 구현합니다.
    true_or_false: bool // bool도 Copy를 구현하므로 문제가 없습니다.
}

fn does_nothing(input: NumberAndBool) {
}

fn main() {
    let number_and_bool = NumberAndBool {
        number: 8,
        true_or_false: true
    };
    
    does_nothing(number_and_bool);
}
