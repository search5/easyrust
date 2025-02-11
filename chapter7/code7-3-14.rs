fn main() {
    let my_vec = vec![8, 9, 10];

    println!("{:?}", my_vec.iter().for_each(||println!("We didn't use the variables at all"))); // ⚠
}
