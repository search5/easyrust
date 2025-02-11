fn main() {
    let (tru, fals) = (true.then(|| 8), false.then(|| 8));
    println!("{:?}, {:?}", tru, fals);
}
