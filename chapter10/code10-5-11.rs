fn main() {
    let mut my_string = String::from("Can I go inside the thread?");

    let handle = std::thread::spawn(move|| {
        println!("{my_string}");
    });
    
    drop(my_string); // ⚠ 스레드 핸들이 있어서 버릴 수 없으므로 동작하지 않습니다.
    
    handle.join();
}
