fn main() { // ⚠
    let mut number_vec = vec![7, 8, 9, 10].into_iter();
    
    let first_two = number_vec.take(2).collect::<Vec<_>>();
    let second_two = number_vec.take(2).collect::<Vec<_>>();
}