use std::sync::Mutex;

fn main() {
    let my_mutex = Mutex::new(5);
    {
        let mut mutex_changer = my_mutex.lock().unwrap();
        *mutex_changer = 6;
    } // mutex_changer가 범위를 벗어납니다. 이제 사라졌습니다. 더는 잠겨있지 않습니다.

    println!("{:?}", my_mutex); // 이제 다음과 같이 표시됩니다. Mutex { data: 6 }
}
