use std::sync::Mutex;

fn main() {
    // 새 Mutex<i32>. mut를 사용할 필요가 없습니다.
    let my_mutex = Mutex::new(5);

    // mutex_changer는 MutexGuard입니다. 값을 변경할 것이므로 mut이어야 합니다.
    let mut mutex_changer = my_mutex.lock().unwrap();

    // 이제 Mutex에 접근할 수 있습니다. 확인하기 위해 my_mutex를 출력해 보겠습니다.
    println!("{:?}", my_mutex); // "Mutex { data: <locked> }"가 출력됩니다.

    // 이제는 my_mutex로 직접 데이터에 접근할 수 없으며,
    // mutex_changer를 통해서만 접근이 가능합니다.
    println!("{:?}", mutex_changer); // 5가 출력됩니다. 6으로 바꾸겠습니다.

    // mutex_changer는 MutexGuard<i32>이므로 *를 사용해 i32 타입의 값을 변경합니다.
    *mutex_changer = 6;

    println!("{:?}", mutex_changer); // 이제 6이 출력됩니다.
}
