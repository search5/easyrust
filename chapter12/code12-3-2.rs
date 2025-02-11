fn main() {
    let my_box = Box::new(1); // 이것은 Box<i32>입니다.
    let an_integer = *my_box; // 이것은 i32입니다.
}
