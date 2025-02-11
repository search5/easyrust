fn just_takes_a_variable<T>(item: T) {} // 아무거나 가져다 놓습니다.

fn main() {
    let my_number = 1; // 이는 i32입니다.

    // 이 함수는 Copy이기 때문에 두 번 사용해도 문제없습니다.
    just_takes_a_variable(my_number);
    just_takes_a_variable(my_number);

    let my_box = Box::new(1); // 이는 Box<i32>입니다.
                              // .clone()이 없으면 두 번째 함수에서 오류가 발생합니다.
    just_takes_a_variable(my_box.clone());
    just_takes_a_variable(my_box); // Box는 Copy가 아니기 때문입니다.
}
