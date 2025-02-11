// 🚧
let my_number = Arc::new(Mutex::new(0));
let my_number1 = Arc::clone(&my_number); // 이 복제본은 스레드 1로 이동합니다.
let my_number2 = Arc::clone(&my_number); // 이 복제본은 스레드 2로 이동합니다.
