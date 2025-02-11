use std::thread;

fn main() {
    thread::spawn(|| {
        // 스레드 작업 수행
    });
    
    thread::spawn(|| {
        // 더 많은 스레드 작업 수행
    });

    // 여기에 join하는 것을 잊지 마세요. join하지 않으면 main()이 완료되기 전에
    // 종료될 수 있습니다.
}
