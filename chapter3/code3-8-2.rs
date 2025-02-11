struct Item {
    number: u8,
}

fn main() {
    let item = Item {
        number: 8,
    };
    let reference_number = &item.number; // 참조되는 숫자 타입은 &u8입니다.
    println!("{}", reference_number == 8); // ⚠ &u8과 u8은 비교할 수 없습니다.
}
