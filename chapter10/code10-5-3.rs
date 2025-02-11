fn main() {
    for _ in 0..10 {
        std::thread::spawn(|| {
            println!("I am printing something");
        });
    }

    for _ in 0..1_000_000 {
        // 프로그램이 "let x = 9"를 100만 번 선언하도록 합니다.
        // main 함수를 종료하기 전에 이 작업을 완료해야 합니다.
        let _x = 9;
    }
}
