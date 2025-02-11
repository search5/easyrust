fn main() {
    let mut my_number = 9;
    my_number += 10;

    let new_vec = vec![8, 9, 10];
    
    let double_vec = new_vec.iter().map(|x| x * 2).collect::<Vec<i32>>();
}
