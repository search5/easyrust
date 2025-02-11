use std::sync::Mutex;
use std::thread;

fn main() {
    // 두 스레드 모두 이것을 사용하므로 Mutex를 사용합니다.
    let my_number = Mutex::new(0);

    // 하나의 스레드만 이것을 사용하므로 Mutex가 필요하지 않습니다.
    let mut regular_mut_number = 0;

    // 누구나 빌림할 수 있도록 변경 불가능하도록 만듭니다.
    let regular_unmut_number = 0;

    thread::scope(|s| {
        s.spawn(|| {
            for _ in 0..3 {
                *my_number.lock().unwrap() += 1;
                regular_mut_number += 1;
                println!("Multiple immutable borrows is fine! {regular_unmut_number}");
            }
        });

        s.spawn(|| {
            for _ in 0..3 {
                *my_number.lock().unwrap() += 1;
                // regular_mut_number += 1; 여전히 이 작업을 할 수 없습니다.
                // regular_mut_number는 두 개의 변경 가능한 빌림입니다.
                println!("Borrowing here too, it's just fine! {regular_unmut_number}");
            }
        });

    });

    println!("my_number: {my_number:?}");
    println!("regular_mut_number: {regular_mut_number}");
}
