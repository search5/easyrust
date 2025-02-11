fn main() {
    let mut big_vec = vec![6; 1000];
    big_vec.push(5);
    
    let mut iterator = big_vec.iter().rev();
    println!("{:?}", iterator.next());
    println!("{:?}", iterator.next());
}
