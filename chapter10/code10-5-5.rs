fn main() {
    for _ in 0..10 {
        let handle = std::thread::spawn(|| {
            println!("I am printing something");
        });

        handle.join(); // 스레드가 완료될 때까지 기다립니다.
    }
}
