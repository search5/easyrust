use std::fmt::Debug; // Debug는 std::fmt::Debug에 있습니다. 이제 'Debug'를 쓰면 됩니다.

fn print_number<T: Debug>(number: T) { // <T: Debug>가 중요한 부분입니다.
    println!("Here is your number: {:?}", number);
}

fn main() {
    print_number(5);
}
