use std::sync::Mutex;

fn main() {
    let my_mutex = Mutex::new(5);
    let mut mutex_changer = my_mutex.lock().unwrap(); // mutex_changer에 잠금이 있음
    let mut other_mutex_changer = my_mutex.lock().unwrap();
    // other_mutex_changer는 잠금을 원합니다.
    // 프로그램이 대기 중입니다.
    // 대기 중...
    // 영원히 대기 중일 겁니다.

    println!("This will never print...");
}
