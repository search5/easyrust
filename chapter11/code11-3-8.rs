use std::sync::{Arc, Mutex};

fn main() {
    let my_number = Arc::new(Mutex::new(0));

    let mut handle_vec = vec![]; // JoinHandles가 여기에 들어갈 것입니다.

    for _ in 0..2 { // 이 작업을 두 번 합니다.
        // 스레드를 시작하기 전에 복제본을 만듭니다.
        let my_number_clone = Arc::clone(&my_number);
        let handle = std::thread::spawn(move || { // 복제본을 넣습니다.
            for _ in 0..10 {
                *my_number_clone.lock().unwrap() += 1;
            }
        });

        // 루프 외부에서 join을 호출할 수 있도록 핸들을 저장합니다.
        // 벡터에 넣지 않으면 여기서 소멸됩니다.
        handle_vec.push(handle);
    }

    // 모든 핸들에서 join을 호출합니다.
    handle_vec.into_iter().for_each(|handle| handle.join().unwrap());
    println!("{my_number:?}");
}
