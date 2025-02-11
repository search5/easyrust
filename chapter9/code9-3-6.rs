use std::cell::RefCell;

#[derive(Debug)]
struct User {
    id: u32,
    year_registered: u32,
    username: String,
    active: RefCell<bool>,
    // 많은 다른 필드
}

fn main() {
    let user_1 = User {
        id: 1,
        year_registered: 2020,
        username: "User 1".to_string(),
        active: RefCell::new(true),
    };

    let borrow_one = user_1.active.borrow_mut(); // 첫 번째 변경 가능한 빌림(문제없음)
    let borrow_two = user_1.active.borrow_mut(); // 두 번째 변경 가능한 빌림(오류 발생)
}
