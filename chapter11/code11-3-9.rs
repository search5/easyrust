use std::sync::{Arc, Mutex};
use std::thread::spawn; // 이제 spawn만 써도 됩니다.

fn make_arc(number: i32) -> Arc<Mutex<i32>> { // Arc에서 Mutex를 만드는 함수입니다.
    Arc::new(Mutex::new(number))
}

fn new_clone(input: &Arc<Mutex<i32>>) -> Arc<Mutex<i32>> {
    // new_clone을 만들 수 있는 함수입니다.
    Arc::clone(&input)
}

// 이제 main()이 더 읽기 쉽습니다.
fn main() {
    let mut handle_vec = vec![]; // 각 핸들이 여기에 들어갈 것입니다.
    let my_number = make_arc(0);

    for _ in 0..2 {
        let my_number_clone = new_clone(&my_number);
        let handle = spawn(move || {
            for _ in 0..10 {
                let mut value_inside = my_number_clone.lock().unwrap();
                *value_inside += 1;
            }
        });
        handle_vec.push(handle); // 핸들이 완성되었으니, 벡터에 넣어주세요.
    }

    handle_vec.into_iter().for_each(|handle| handle.join().unwrap());
    // 각자 대기합니다.
    
    println!("{my_number:?}");
}
