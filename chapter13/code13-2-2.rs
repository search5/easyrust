fn main() {
    let value = 7;
    let reference = &7;
    println!("{}", value == *reference);
}
