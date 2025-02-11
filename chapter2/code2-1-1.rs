fn main() {
    let my_number = 15; // 이는 i32입니다.
    let single_reference = &my_number; // 이는 &i32입니다.
    let double_reference = &single_reference; // 이는 &&i32입니다.
    let five_references = &&&&&my_number; // 이는 &&&&&i32입니다.
}
