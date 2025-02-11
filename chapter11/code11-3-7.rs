use std::sync::{Arc, Mutex};

fn main() {
    let my_number = Arc::new(Mutex::new(0));
    let my_number1 = Arc::clone(&my_number);
    let my_number2 = Arc::clone(&my_number);

    let thread1 = std::thread::spawn(move || {
        // 복제본만 스레드 1로 이동합니다.
        for _ in 0..10 {
            *my_number1.lock().unwrap() += 1; // Mutex를 잠그고 값을 변경합니다.
        }
    });

    let thread2 = std::thread::spawn(move || {
        // 복제본만 스레드 2로 이동합니다.
        for _ in 0..10 {
            *my_number2.lock().unwrap() += 1;
        }
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
    
    println!("Value is: {my_number:?}");
    println!("Exiting the program");
}
