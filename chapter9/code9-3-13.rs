use std::sync::Mutex;

fn main() {
    let my_mutex = Mutex::new(5);

    for _ in 0..100 {
        *my_mutex.lock().unwrap() += 1; // 100번을 잠금 및 잠금 해제
    }
    
    println!("{:?}", my_mutex);
}
