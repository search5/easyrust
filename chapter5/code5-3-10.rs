// ⚠
fn get_fourth(input: &Vec<i32>) -> i32 {
    let fourth = input.get(3).unwrap();
    *fourth
}

fn main() {
    let my_vec = vec![9, 0, 10];
    let fourth = get_fourth(&my_vec);
}
