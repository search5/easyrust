fn main() {
    let mut my_vec = vec![0; 600_000];
    
    for i in 0..600000 {
        my_vec.remove(0);
    }
}
