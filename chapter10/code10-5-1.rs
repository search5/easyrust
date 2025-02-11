fn main() {
    std::thread::spawn(|| {
        println!("I am printing something");
    });
}
