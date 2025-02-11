fn main() {
    let slice1 = "Hello!";
    println!("Slice1 is {} bytes and also {} characters.", slice1.len(), slice1.chars().count());
    let slice2 = "안녕!";
    println!("Slice2 is {} bytes but only {} characters.", slice2.len(), slice2.chars().count());
}
