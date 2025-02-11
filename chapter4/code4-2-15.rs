fn main() {
    let my_vec = vec![2, 3, 4];
    
    for index in 0..10 {
        match my_vec.get(index) {
            Some(number) => println!("The number is: {number}"),
            None => {}
        }
    }
}
