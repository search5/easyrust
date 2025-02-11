fn main() {
    let new_vec = vec![8, 9, 10];
    let double_vec = new_vec
        .iter()
        .inspect(|first_item| {
            println!("The item is: {first_item}");
            match **first_item % 2 { // 첫 번째 항목은 &&i32이므로 **를 사용합니다.
                0 => println!("It is even."),
                _ => println!("It is odd."),
            }
        
            println!("In binary it is {:b}.", first_item);
        })
    .map(|x| x * 2)
    .collect::<Vec<i32>>();
}
