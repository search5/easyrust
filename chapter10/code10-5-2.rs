fn main() {
    for _ in 0..10 { // 10개의 스레드 설정
        std::thread::spawn(|| {
            println!("I am printing something");
        });
    } // 이제 스레드가 시작됩니다.
} // 여기서 main()이 끝나기 전에 몇 개나 완료할 수 있을까요?
