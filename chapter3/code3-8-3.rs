struct Item {
    number: u8,
}

fn main() {
    let item = Item {
        number: 8,
    };
    let reference_item = &item;
    println!("{}", reference_item.number == 8);
    // *reference_item.number를 쓸 필요가 없습니다.
}
