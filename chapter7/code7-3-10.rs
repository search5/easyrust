fn main() {
    let num_vec = vec![10, 9, 8];
    
    num_vec
        .iter()
        .enumerate()
        .map(|(index, number)| println!("Index number {} has number {}", index, number));
}
