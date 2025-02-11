// ->가 없으므로 아무것도 반환하지 않습니다.
// 숫자가 복사 타입이 아니면 가져가서 다시 사용할 수 없습니다.
fn prints_number(number: i32) {
    println!("{}", number);
}

fn main() {
    let my_number = 8;
    // 8을 출력합니다. prints_number는 my_number의 사본을 가져옵니다.
    prints_number(my_number);
    // 8을 다시 출력합니다. my_number는 복사 타입이므로 문제없습니다.
    prints_number(my_number);
}
