struct Item {
    number: u8,
}

impl Item {
    fn compare_number(&self, other_number: u8) {
        // self를 참조합니다.
        println!("Are {} and {} equal? {}", self.number, other_number, self.number == other_number);
        // *self.number를 쓸 필요가 없습니다.
    }
}

fn main() {
    let item = Item {
        number: 8,
    };
    let reference_item = &item; // 이것은 &Item 타입입니다.
    let reference_item_two = &reference_item; // 이것은 &&Item 타입입니다.
    item.compare_number(8); // 메서드가 동작합니다.
    reference_item.compare_number(8); // 여기에서도 동작합니다.
    reference_item_two.compare_number(8); // 그리고 여기도요.
}
