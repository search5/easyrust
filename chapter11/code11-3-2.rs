fn main() {
    let handle = std::thread::spawn(|| {
        println!("The thread is working!") // 스레드 테스트
    });
    
    handle.join().unwrap(); // 스레드가 완료될 때까지 여기서 대기합니다.
    println!("Exiting the program");
}
