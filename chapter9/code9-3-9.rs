use std::sync::Mutex;

fn main() {
    let my_mutex = Mutex::new(5);
    let mut mutex_changer = my_mutex.lock().unwrap();
    *mutex_changer = 6;
    drop(mutex_changer); // mutex_changer를 버립니다. 이제 사라졌습니다.
                         // my_mutex가 잠금 해제되었습니다.

    println!("{:?}", my_mutex); // 이제 다음과 같이 표시됩니다. Mutex { data: 6 }
}
